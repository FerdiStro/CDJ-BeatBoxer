package com.FerdiStro.memory;

import com.FerdiStro.memory.objects.ReceivedData;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import java.io.File;
import java.io.RandomAccessFile;
import java.nio.ByteOrder;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;
import java.util.function.Consumer;

public class HighPerfReader implements Runnable {
    private static final Logger log = LogManager.getLogger();

    private final String FILE_PATH;
    private final int FILE_SIZE;

    private final Consumer<ReceivedData> dataCallback;
    private volatile boolean running = true;

    public HighPerfReader(String filePath, int fileSize, Consumer<ReceivedData> dataCallback) {
        this.FILE_PATH = filePath;
        this.FILE_SIZE = fileSize;
        this.dataCallback = dataCallback;

    }


    public void stop() {
        this.running = false;
    }

    @Override
    public void run() {
        log.info("SharedMemory-Reader started, waiting on Memory");
        File file = new File(FILE_PATH);
        RandomAccessFile raf = null;
        FileChannel channel = null;
        MappedByteBuffer buffer = null;

        while (running) {
            try {
                if (file.exists() && file.length() >= FILE_SIZE) {
                    raf = new RandomAccessFile(file, "r");
                    channel = raf.getChannel();
                    buffer = channel.map(FileChannel.MapMode.READ_ONLY, 0, FILE_SIZE);
                    buffer.order(ByteOrder.LITTLE_ENDIAN);
                    log.info("SharedMemory found");
                    break;
                } else {
                    Thread.sleep(1000);
                    Thread.onSpinWait();
                }
            } catch (Exception e) {
                log.error("Error on staring Reader:", e);
                try {
                    Thread.sleep(1000);
                } catch (InterruptedException ignored) {
                    //ignore
                }
            }
        }

        if (buffer == null) return;

        long lastSequence = -1;

        while (running) {
            try {
                long currentSequence = buffer.getLong(0);
                if (currentSequence > lastSequence) {
                    if (dataCallback != null && lastSequence != -1) {
                        dataCallback.accept(new ReceivedData(buffer));
                    }
                    lastSequence = currentSequence;
                } else {
                    Thread.onSpinWait();
                }
            } catch (Exception e) {
                log.error("Error in Hot-Loop", e);
            }
        }

        try {
            if (channel != null) channel.close();
            if (raf != null) raf.close();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}