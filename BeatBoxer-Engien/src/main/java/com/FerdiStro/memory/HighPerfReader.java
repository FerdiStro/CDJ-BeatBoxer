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

    private final String filePath;
    private final int fileSize;

    private final Consumer<ReceivedData> dataCallback;
    private volatile boolean running = true;

    private FileChannel channel = null;
    private MappedByteBuffer buffer = null;

    public HighPerfReader(String filePath, int fileSize, Consumer<ReceivedData> dataCallback) {
        this.filePath = filePath;
        this.fileSize = fileSize;
        this.dataCallback = dataCallback;
    }


    public void stop() {
        this.running = false;
    }

    private RandomAccessFile waitingForRaF() {
        File file = new File(filePath);
        RandomAccessFile raf = null;

        while (running) {
            try {
                if (file.exists() && file.length() >= fileSize) {
                    raf = new RandomAccessFile(file, "r");
                    channel = raf.getChannel();
                    buffer = channel.map(FileChannel.MapMode.READ_ONLY, 0, fileSize);
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
                } catch (InterruptedException interruptedException) {
                    log.error(interruptedException);
                    Thread.currentThread().interrupt();
                }
            }
        }

        return raf;

    }


    @Override
    public void run() {
        log.info("SharedMemory-Reader started, waiting on Memory");

        RandomAccessFile raF = waitingForRaF();

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
            if (raF != null) raF.close();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}