package com.FerdiStro.drum;

import com.FerdiStro.cdj.modes.AbstractMode;
import com.FerdiStro.memory.SharedMemoryProvider;

public class DrumMachine implements DrumMachineCommand {

    private final AbstractMode mode;

    public DrumMachine(AbstractMode mode) {
        mode.setDrumMachineCommandLine(this);
        SharedMemoryProvider.getInstance().addMemoryUpdateListeners(mode);
        this.mode = mode;

    }

    @Override
    public void onCommand() {

    }
}
