package com.FerdiStro.memory;

import com.FerdiStro.memory.exceptions.ByteSizeValidationException;
import lombok.Getter;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import java.nio.MappedByteBuffer;


public class TransferObject {
    @Getter
    private final double bpm;
    @Getter
    private final byte smallCounter;
    private final long totalCounter;
    protected static final Logger log = LogManager.getLogger();




    public TransferObject(double bpm, byte smallCounter, long totalCounter) {
        byte maxSize = 0b100;
        if (smallCounter > maxSize) {
            throw new ByteSizeValidationException("Small-Counter", maxSize, smallCounter);
        }
        this.bpm = bpm;
        this.smallCounter = smallCounter;
        this.totalCounter = totalCounter;
    }

    private static final int POSITION_BPM = 8;
    private static final int POSITION_SMALL_COUNTER = 16;
    private static final int POSITION_TOTAL_COUNTER = 24;

    public void writeMappedByteBuffer(MappedByteBuffer buffer, long sequence) {
        if (buffer.isReadOnly()) {
            log.error("Buffer is read only");
            return;
        }
        buffer.position(0);
        buffer.putDouble(POSITION_BPM, bpm);
        buffer.put(POSITION_SMALL_COUNTER, smallCounter);
        buffer.putLong(POSITION_TOTAL_COUNTER, totalCounter);

        buffer.putLong(0, sequence);
    }

}
