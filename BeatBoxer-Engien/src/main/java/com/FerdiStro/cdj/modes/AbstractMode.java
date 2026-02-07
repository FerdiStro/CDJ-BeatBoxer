package com.FerdiStro.cdj.modes;

import com.FerdiStro.LogUtils;
import com.FerdiStro.cdj.Modifier;
import com.FerdiStro.memory.SharedMemoryProvider;
import com.FerdiStro.memory.TransferObject;
import lombok.Getter;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

public abstract class AbstractMode {

    public abstract String getName();

    public abstract void startUp();

    public abstract double getCurrentBpm();

    public abstract boolean isMaster();

    public abstract void modifyBpm(Modifier modifier, float value);

    protected SharedMemoryProvider memoryProvider;

    protected static final Logger log = LogManager.getLogger();


    @Getter
    private Long totalCounter = 1L;

    @Getter
    private byte smallCounter = 0;

    /**
     * Beat grid is displayed by a long max 64 Bit long. Position can only increase, after 64. place it will be reset to the first postion.
     * Ideal for presenting 64 beats long Sound grid.
     * SmallCount just 4 Bits big counting.
     * Only Increasing position
     */
    private void movePosition() {
        totalCounter <<= 1;
        if (totalCounter == 0) {
            totalCounter = 1L;
        }
        smallCounter = (byte) ((smallCounter + 1) & 3);
    }


    public void init() {
        log.info(LogUtils.LINE_SEPARATOR);
        log.info("Start:  {}", getName());
        log.info(LogUtils.LINE_SEPARATOR);
        this.memoryProvider = SharedMemoryProvider.getInstance();
    }




    public void onBeat(int barPosition) {
        this.smallCounter = (byte) (barPosition - 1);
        TransferObject transferObject = new TransferObject(getCurrentBpm(), getSmallCounter(), getTotalCounter());
        this.memoryProvider.writeToMemory(transferObject);
    }

    public void printAnalytics() {
        log.info(LogUtils.LINE_SEPARATOR);
        log.info("Bpm: {}", getCurrentBpm());
        log.info("Master: {}", isMaster());
        log.info(LogUtils.LINE_SEPARATOR);
    }


}
