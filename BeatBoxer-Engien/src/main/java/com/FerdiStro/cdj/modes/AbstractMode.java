package com.FerdiStro.cdj.modes;

import com.FerdiStro.LogUtils;
import com.FerdiStro.drum.DrumMachine;
import com.FerdiStro.drum.command.DrumCommand;
import com.FerdiStro.drum.command.DrumCommandObject;
import com.FerdiStro.memory.SharedMemoryProvider;
import com.FerdiStro.memory.bus.MemoryUpdateCommand;
import com.FerdiStro.memory.bus.MemoryUpdateListener;
import com.FerdiStro.memory.objects.ReceivedData;
import com.FerdiStro.memory.objects.TransferObject;
import lombok.Getter;
import lombok.Setter;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

public abstract class AbstractMode implements MemoryUpdateListener {

    protected static final Logger log = LogManager.getLogger();
    protected SharedMemoryProvider memoryProvider;
    /**
     * Beat grid is displayed by a long max 64 Bit long. Position can only increase, after 64. place it will be reset to the first postion.
     * Ideal for presenting 64 beats long Sound grid.
     * SmallCount just 4 Bits big counting.
     * Only Increasing position
     */
    int syncBarPosition = 1;
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

    private Boolean movePosition(int barPosition) {
        boolean sync = false;
        if (barPosition == syncBarPosition) {
            totalCounter <<= 1;
            if (totalCounter == 0) {
                totalCounter = 1L;
            }
            smallCounter = (byte) ((smallCounter + 1) & 7);

            syncBarPosition += 1;

            if (syncBarPosition == 5) {
                syncBarPosition = 1;
            }
            sync = true;
        }

        return sync;
    }

    public void init() {
        log.info(LogUtils.LINE_SEPARATOR);
        log.info("Start:  {}", getName());
        log.info(LogUtils.LINE_SEPARATOR);
        this.memoryProvider = SharedMemoryProvider.getInstance();
    }

    public void onMasterChange() {
        this.sendMemoryUpdate();
    }

    public void onTempoChange() {
        this.sendMemoryUpdate();
    }

    public void onBeat(int barPosition) {
        if (Boolean.FALSE.equals(movePosition(barPosition))) {
            return;
        }
        this.drumMachineCommandLine.onBeat(this.smallCounter);
        this.sendMemoryUpdate();
    }

    public void printAnalytics() {
        log.info(LogUtils.LINE_SEPARATOR);
        log.info("Bpm: {}", getCurrentBpm());
        log.info("Master: {}", isMaster());
        log.info(LogUtils.LINE_SEPARATOR);
    }

    public void sendMemoryUpdate() {
        TransferObject transferObject = new TransferObject(getCurrentBpm(), getSmallCounter(), getTotalCounter(), isMaster(), drumMachineCommandLine.getSmallGrid());
        this.memoryProvider.writeToMemory(transferObject);
    }

    @Override
    public void onMemoryUpdate(MemoryUpdateCommand command) {

        ReceivedData lastData = memoryProvider.getLastData();
        switch (command) {
            case SMALL_BEAT -> this.smallBeat = !this.smallBeat;
            case ADD_BEAT_SMALL -> {
                DrumCommandObject drumCommandObject = new DrumCommandObject(DrumCommand.ADD_SOUND, lastData.getSmallCounter(), lastData.getSelectedSoundPath());
                drumMachineCommandLine.onCommand(drumCommandObject);

            }
            case REMOVE_BEAT_SMALL -> {
                DrumCommandObject drumCommandObject = new DrumCommandObject(DrumCommand.REMOVE_SOUND, lastData.getSmallCounter(), lastData.getRemoveSoundPath());
                drumMachineCommandLine.onCommand(drumCommandObject);
            }
            default -> onMemoryUpdateImpl(command);
        }
    }
}
