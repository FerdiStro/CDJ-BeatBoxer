package com.FerdiStro.cdj.modes;


import com.FerdiStro.cdj.Modifier;
import com.FerdiStro.cdj.VirtualDevice;


public class ConnectMode extends AbstractMode {


    private static final String MODE_NAME = "CDJ-connect-mode";


    private VirtualDevice virtualDevice = null;

    public ConnectMode() {
        super.printStartUpSequence();
    }

    @Override
    public String getName() {
        return MODE_NAME;
    }

    @Override
    public void startUp() {
        VirtualDevice virtualDevice = VirtualDevice.getInstance();
        virtualDevice.addBeatListener(beat -> onBeat());
        this.virtualDevice = virtualDevice;
    }


    @Override
    public double getCurrentBpm() {
        return virtualDevice.getMasterBpm();
    }

    @Override
    public boolean isMaster() {
        return virtualDevice.istMaster();
    }

    @Override
    public void modifyBpm(Modifier modifier, float value) {

    }
}
