package com.FerdiStro.cdj.modes;

public class OfflineMode extends AbstractMode {

    private static final String MODE_NAME = "Offline-mode";


    public OfflineMode() {
        super.printStartUpSequence();
    }


    @Override
    public String getName() {
        return MODE_NAME;
    }

    @Override
    public void startUp() {
        //todo: offlineMode
    }
}
