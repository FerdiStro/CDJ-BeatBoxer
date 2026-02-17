package com.FerdiStro.drum.modes;


import com.FerdiStro.cdj.VirtualDevice;
import com.FerdiStro.memory.bus.MemoryUpdateCommand;
import org.deepsymmetry.beatlink.Beat;
import org.deepsymmetry.beatlink.DeviceUpdate;
import org.deepsymmetry.beatlink.MasterListener;


public class ConnectMode extends AbstractMode {


    private static final String MODE_NAME = "CDJ-connect-mode";
    private VirtualDevice virtualDevice = null;


    public ConnectMode() {
        super.init();
    }


    private void becomeMaster() {
        virtualDevice.becomeMaster();
        if (virtualDevice.istMaster()) {
            log.info("Became master immediately (no conflict).");
            ConnectMode.super.onMasterChange();
        } else {
            log.info("Request Master waiting for answer");
            Thread waitMasterUpdate = new Thread(() -> {
                int retries = 0;
                while (retries < 10) {
                    if (isMaster()) {
                        log.info("SUCCESS: VirtualCdj is now Tempo Master!");
                        ConnectMode.super.onMasterChange();
                        return;
                    }
                    try {
                        Thread.sleep(50);
                        Thread.onSpinWait();
                    } catch (InterruptedException e) {
                        break;
                    }
                    retries++;
                }
                log.error("Timed out waiting for VirtualCdj to become master.");
            });
            waitMasterUpdate.setName("WaitM");
            waitMasterUpdate.start();
        }
    }

    @Override
    public String getName() {
        return MODE_NAME;
    }


    @Override
    public void startUp() {
        VirtualDevice virtualDevice = VirtualDevice.getInstance();
        virtualDevice.addBeatListener(new MasterListener() {
            @Override
            public void masterChanged(DeviceUpdate update) {
                log.debug(update.toString());
                ConnectMode.super.onMasterChange();
            }

            @Override
            public void tempoChanged(double tempo) {
                ConnectMode.super.onTempoChange();
            }

            @Override
            public void newBeat(Beat beat) {
                if (!beat.isTempoMaster()) return;
                onBeat(beat.getBeatWithinBar());
            }
        });


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
    public void onMemoryUpdateImpl(MemoryUpdateCommand command) {
        switch (command) {
            case BECOME_MASTER -> becomeMaster();
        }


    }


}
