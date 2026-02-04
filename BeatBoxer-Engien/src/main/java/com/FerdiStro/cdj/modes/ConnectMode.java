package com.FerdiStro.cdj.modes;


import com.FerdiStro.cdj.VirtualDevice;

public class ConnectMode extends AbstractMode {


    private static final String MODE_NAME = "CDJ-connect-mode";


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
        virtualDevice.addBeatListener(ConnectMode.super::onBeat);

    }
}
