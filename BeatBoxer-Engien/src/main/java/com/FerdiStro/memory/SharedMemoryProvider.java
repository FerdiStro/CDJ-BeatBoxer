package com.FerdiStro.memory;

import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import java.io.File;
import java.io.RandomAccessFile;
import java.nio.ByteOrder;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;


public class SharedMemoryProvider {

    protected static final Logger log = LogManager.getLogger();


    private static SharedMemoryProvider INSTANCE = null;

    private static final String FILE_NAME = "fromEngien_shm.bin";
    private static final Integer FILE_LENGTH = 4096;

    private RandomAccessFile randomAccessFile;
    private FileChannel channel;
    private MappedByteBuffer buffer;
    private long counter = 0;


    public void start() {

        // Writing part
        File file = new File(FILE_NAME);
        try {
            if (file.createNewFile()) {
                log.info("{}  created!", FILE_NAME);
            } else {
                log.error("Can not create {}", FILE_NAME);
            }

            this.randomAccessFile = new RandomAccessFile(file, "rw");
            this.randomAccessFile.setLength(FILE_LENGTH);

            this.channel = this.randomAccessFile.getChannel();

            this.buffer = this.channel.map(FileChannel.MapMode.READ_WRITE, 0, FILE_LENGTH);
            this.buffer.order(ByteOrder.LITTLE_ENDIAN);

            byte[] reset_bytes = new byte[FILE_LENGTH];
            while (buffer.remaining() > 0) {
                int len = Math.min(buffer.remaining(), reset_bytes.length);
                buffer.put(reset_bytes, 0, len);
            }

        } catch (Exception e) {
            log.error("Error while creating SharedMemoryProvider", e);
            e.printStackTrace();
            close();
        }

        //reading part


    }


    private SharedMemoryProvider() {

    }

    public static SharedMemoryProvider getInstance() {
        if (INSTANCE == null) {
            INSTANCE = new SharedMemoryProvider();
        }
        return INSTANCE;
    }


    public void writeToMemory(TransferObject transferObject) {
        if (buffer == null) {
            log.error("ERROR: SharedMemoryProvider, buffer == null");
            return;
        }
        counter++;
        transferObject.writeMappedByteBuffer(buffer, counter);

        //todo: remove only for testing:
        double checkBpm = buffer.getDouble(8);
        if (checkBpm == transferObject.getBpm()) {
            log.info("Buffer reader works");
        }
    }


    public void close() {
        try {
            if (channel != null) channel.close();
            if (randomAccessFile != null) randomAccessFile.close();
            buffer = null;
        } catch (Exception e) {
            log.error("Error closing shared memory", e);
        }
    }


}
