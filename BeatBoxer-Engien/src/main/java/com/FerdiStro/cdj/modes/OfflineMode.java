package com.FerdiStro.cdj.modes;

import com.FerdiStro.LogUtils;
import com.FerdiStro.cdj.Modifier;

public class OfflineMode extends AbstractMode {

    private static final String MODE_NAME = "Offline-mode";
    private static final double DEFAULT_BPM = 120.00;

    public OfflineMode() {
        super.printStartUpSequence();
    }


    @Override
    public String getName() {
        return MODE_NAME;
    }

    @Override
    public void startUp() {
        log.error(LogUtils.NOT_IMPLEMENTED);
    }

    @Override
    public double getCurrentBpm() {
        return DEFAULT_BPM;
    }

    @Override
    public boolean isMaster() {
        return true;
    }

    @Override
    public void modifyBpm(Modifier modifier, float value) {
        log.error(LogUtils.NOT_IMPLEMENTED);
    }
}
