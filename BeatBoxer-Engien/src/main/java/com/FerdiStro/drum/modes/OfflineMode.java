package com.FerdiStro.drum.modes;

import com.FerdiStro.drum.modes.pattern.AbstractPatternSystem;
import com.FerdiStro.drum.modes.pattern.OfflinePatternSystem;

import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.TimeUnit;

public class OfflineMode extends AbstractMode {

    private static final String MODE_NAME = "Offline-mode";
    private static final double DEFAULT_BPM = 120.00;
    private ScheduledExecutorService scheduler;


    private byte bar = 0;

    public OfflineMode() {
        super.init();
    }

    @Override
    public String getName() {
        return MODE_NAME;
    }

    @Override
    public void startUp() {
        stopScheduler();

        scheduler = Executors.newSingleThreadScheduledExecutor(r -> {
            Thread t = new Thread(r, "Off-Drum-Thread");
            t.setDaemon(true);
            return t;
        });
        long periodMs = (long) (60000.0 / getCurrentBpm());
        scheduler.scheduleAtFixedRate(this::onBeat, 0, periodMs, TimeUnit.MILLISECONDS);

    }

    private void onBeat() {
        super.onBeat(bar + 1);
        bar = (byte) ((bar + 1) & 3);
    }

    @Override
    protected AbstractPatternSystem getImplementedPatternSystem() {
        return new OfflinePatternSystem();
    }


    @Override
    public double getCurrentBpm() {
        return DEFAULT_BPM;
    }

    @Override
    public boolean isMaster() {
        return true;
    }


    public void stopScheduler() {
        if (scheduler != null && !scheduler.isShutdown()) {
            scheduler.shutdownNow();
            scheduler = null;
        }
    }

}
