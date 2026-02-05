package com.FerdiStro.cdj.modes;

import com.FerdiStro.LogUtils;
import com.FerdiStro.cdj.Modifier;
import lombok.Getter;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

public abstract class AbstractMode {

    public abstract String getName();

    public abstract void startUp();

    public abstract double getCurrentBpm();

    public abstract boolean isMaster();

    public abstract void modifyBpm(Modifier modifier, float value);

    protected static final Logger log = LogManager.getLogger();

    @Getter
    private Long totalCounter = 1L;

    @Getter
    private byte smallCounter = 0b001;

    /**
     * Beat grid is displayed by a long max 64 Bit long. Position can only increase, after 64. place it will be reset to the first postion.
     * Ideal for presenting 64 beats long Sound grid.
     * SmallCount just 4 Bits big counting.
     * Only Increasing position
     */
    public void movePosition() {
        if ((totalCounter & (1L << 63)) == 0) {
            totalCounter <<= 1;
        }
        if ((smallCounter & 0b1000) == 0) {
            smallCounter <<= 1;
        }
    }

    public void printStartUpSequence() {
        log.info(LogUtils.LINE_SEPARATOR);
        log.info("Start:  {}", getName());
        log.info(LogUtils.LINE_SEPARATOR);
    }


    public void onBeat() {
    }

    public void printAnalytics() {
        log.info(LogUtils.LINE_SEPARATOR);
        log.info("Bpm: {}", getCurrentBpm());
        log.info("Master: {}", isMaster());
        log.info(LogUtils.LINE_SEPARATOR);
    }


}
