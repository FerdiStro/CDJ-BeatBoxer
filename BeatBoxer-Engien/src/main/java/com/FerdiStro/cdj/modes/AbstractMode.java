package com.FerdiStro.cdj.modes;

import com.FerdiStro.LogUtils;
import com.FerdiStro.drum.DrumMachine;
import com.FerdiStro.memory.SharedMemoryProvider;
import com.FerdiStro.memory.bus.MemoryUpdateCommand;
import com.FerdiStro.memory.bus.MemoryUpdateListener;
import com.FerdiStro.memory.objects.TransferObject;
import lombok.Getter;
import lombok.Setter;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

public abstract class AbstractMode implements MemoryUpdateListener {

    protected static final Logger log = LogManager.getLogger();
    private static final String SAMPLE_KICK = "/home/ferdinoond/CDJ-BeatBoxer/KICK_20.wav";
    protected SharedMemoryProvider memoryProvider;
    @Getter
    private boolean smallBeat = true;
    @Setter
    private DrumMachine drumMachineCommandLine;
    @Getter
    private Long totalCounter = 1L;
    @Getter
    private byte smallCounter = 0;

    public abstract String getName();

    public abstract void startUp();

    public abstract double getCurrentBpm();

    public abstract boolean isMaster();

    /**
     * Can be overridden for Mode specific MemoryUpdateCommand
     *
     * @param command
     */
    public void onMemoryUpdateImpl(MemoryUpdateCommand command) {
    }

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

    public void onMasterChange() {
        this.sendTransferObject();
    }

    public void onTempoChange() {
        this.sendTransferObject();
    }

    public void onBeat(int barPosition) {
        this.smallCounter = (byte) (barPosition - 1);
        this.drumMachineCommandLine.onBeat(this.smallCounter);
        this.sendTransferObject();
    }

    public void printAnalytics() {
        log.info(LogUtils.LINE_SEPARATOR);
        log.info("Bpm: {}", getCurrentBpm());
        log.info("Master: {}", isMaster());
        log.info(LogUtils.LINE_SEPARATOR);
    }

    private void sendTransferObject() {
        TransferObject transferObject = new TransferObject(getCurrentBpm(), getSmallCounter(), getTotalCounter(), isMaster());
        this.memoryProvider.writeToMemory(transferObject);
    }

    @Override
    public void onMemoryUpdate(MemoryUpdateCommand command) {
        switch (command) {
            case SMALL_BEAT -> this.smallBeat = !this.smallBeat;
            default -> onMemoryUpdateImpl(command);
        }
    }
}
