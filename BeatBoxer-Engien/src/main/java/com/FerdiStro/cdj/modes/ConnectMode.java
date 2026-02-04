package com.FerdiStro.cdj.modes;


import com.FerdiStro.cdj.VirtualDevice;
import org.deepsymmetry.beatlink.Beat;
import org.deepsymmetry.beatlink.BeatListener;

public class ConnectMode extends  AbstractMode{


    private static final String MODE_NAME = "CDJ-connect-mode";


    public ConnectMode(){
        super.printStartUpSequence();
    }

    @Override
    public String getName() {
        return MODE_NAME;
    }

    @Override
    public void startUp() {
        VirtualDevice virtualDevice = VirtualDevice.getInstance();
        virtualDevice.addBeatListener(new BeatListener() {
            @Override
            public void newBeat(Beat beat) {
                ConnectMode.super.onBeat();
            }
        });

    }
}
