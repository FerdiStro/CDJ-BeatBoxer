package com.FerdiStro.memory;

import com.FerdiStro.memory.bus.MemoryUpdateCommand;
import com.FerdiStro.memory.bus.MemoryUpdateListener;
import com.FerdiStro.memory.objects.ReceivedData;
import com.FerdiStro.memory.objects.TransferObject;
import lombok.Getter;
import lombok.Setter;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import java.io.File;
import java.io.RandomAccessFile;
import java.nio.ByteOrder;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;
import java.util.ArrayList;
import java.util.List;


public class SharedMemoryProvider {

    protected static final Logger log = LogManager.getLogger();
    private static final String FROM_ENGIEN_SHM = "fromEngien_shm.bin";
    private static final String TO_ENGIEN_SHM = "toEngien_shm.bin";
    private static final Integer FILE_LENGTH = 4096;
    private static SharedMemoryProvider INSTANCE = null;
    private final List<MemoryUpdateListener> memoryUpdateListeners = new ArrayList<>();
    Thread readerThread = null;
    private RandomAccessFile fromEngienRandomAccessFile;
    private FileChannel fromEngienChannel;
    private MappedByteBuffer fromEngienBuffer;
    private long fromEngienCounter = 0;
    @Getter
    private ReceivedData lastData;
    @Setter
    private Boolean stopReading = true;

    private SharedMemoryProvider() {

    }

    public static SharedMemoryProvider getInstance() {
        if (INSTANCE == null) {
            INSTANCE = new SharedMemoryProvider();
        }
        return INSTANCE;
    }

    public void addMemoryUpdateListeners(MemoryUpdateListener updateListener) {
        log.info("Add new MemoryUpdateListener");
        this.memoryUpdateListeners.add(updateListener);
    }

    public void notifyMemoryUpdateListeners(MemoryUpdateCommand command) {
        for (MemoryUpdateListener listener : this.memoryUpdateListeners) {
            listener.onMemoryUpdate(command);
        }
    }

    public void start() {
        startWriting();
        startReaderThread();
    }

    private void startWriting() {
        File fromEngienFile = new File(FROM_ENGIEN_SHM);
        try {

            this.fromEngienRandomAccessFile = new RandomAccessFile(fromEngienFile, "rw");
            this.fromEngienRandomAccessFile.setLength(FILE_LENGTH);
            this.fromEngienChannel = this.fromEngienRandomAccessFile.getChannel();

            this.fromEngienBuffer = this.fromEngienChannel.map(FileChannel.MapMode.READ_WRITE, 0, FILE_LENGTH);
            this.fromEngienBuffer.order(ByteOrder.LITTLE_ENDIAN);

            byte[] restBytes = new byte[FILE_LENGTH];
            while (fromEngienBuffer.remaining() > 0) {
                int len = Math.min(fromEngienBuffer.remaining(), restBytes.length);
                fromEngienBuffer.put(restBytes, 0, len);
            }
            fromEngienBuffer.position(0);

        } catch (Exception e) {
            log.error("Error while creating SharedMemoryProvider", e);
            e.printStackTrace();
            close();
        }
    }

    private void startReaderThread() {
        HighPerfReader reader = new HighPerfReader(TO_ENGIEN_SHM, FILE_LENGTH, this::updateHandler);
        readerThread = new Thread(reader);
        readerThread.setPriority(Thread.MAX_PRIORITY);
        readerThread.setName("SHMR");
        readerThread.start();
    }

    private void updateHandler(ReceivedData data) {
        if (Boolean.TRUE.equals(stopReading)) return;

        //reading is active:
        log.debug(data);
        this.lastData = data;

        if (data.isRemoveSoundOnSmallBeat()) {
            if (!data.getRemoveSoundPath().isBlank()) {
                notifyMemoryUpdateListeners(MemoryUpdateCommand.REMOVE_BEAT_SMALL);
            }
            return;
        }

        if (data.isAddSoundOnSmallBeat()) {
            if (!data.getSelectedSoundPath().isBlank()) {
                notifyMemoryUpdateListeners(MemoryUpdateCommand.ADD_BEAT_SMALL);
            }
            return;
        }

        if (data.isBecomeMaster()) {
            notifyMemoryUpdateListeners(MemoryUpdateCommand.BECOME_MASTER);
            return;
        }

        if (data.isOnShootModus()) {
            notifyMemoryUpdateListeners(MemoryUpdateCommand.ON_SHOOT_MODUS);
            return;
        }


        if (data.isIncreaseBpm()) {
            notifyMemoryUpdateListeners(MemoryUpdateCommand.INCREASE_BPM);
            return;
        }

        if (data.isDecreaseBpm()) {
            notifyMemoryUpdateListeners(MemoryUpdateCommand.DECREASE_BPM);
            return;
        }

        if (data.isKnobEcho()) {
            notifyMemoryUpdateListeners(MemoryUpdateCommand.EFFECT_ECHO);
            return;
        }

        if (data.isKnobReverb()) {
            notifyMemoryUpdateListeners(MemoryUpdateCommand.EFFECT_REVERB);
            return;
        }
        if (data.isKnobDistortion()) {
            notifyMemoryUpdateListeners(MemoryUpdateCommand.EFFECT_DISTORTION);
            return;
        }
        notifyMemoryUpdateListeners(MemoryUpdateCommand.DEFAULT);
    }


    public void writeToMemory(TransferObject transferObject) {
        if (fromEngienBuffer == null) {
            log.error("ERROR: SharedMemoryProvider, buffer == null");
            return;
        }
        fromEngienCounter++;
        transferObject.writeMappedByteBuffer(fromEngienBuffer, fromEngienCounter);
    }


    public void close() {
        try {
            if (fromEngienChannel != null) fromEngienChannel.close();
            if (fromEngienRandomAccessFile != null) fromEngienRandomAccessFile.close();
            fromEngienBuffer = null;
        } catch (Exception e) {
            log.error("Error closing shared memory", e);
        }
    }
}
