package com.FerdiStro.cdj.modes;


import com.FerdiStro.cdj.Modifier;
import com.FerdiStro.cdj.VirtualDevice;
import org.deepsymmetry.beatlink.data.*;


public class ConnectMode extends AbstractMode {


    private static final String MODE_NAME = "CDJ-connect-mode";


    private VirtualDevice virtualDevice = null;

    public ConnectMode() {
        super.init();
    }

    @Override
    public String getName() {
        return MODE_NAME;
    }


    @Override
    public void startUp() {
        VirtualDevice virtualDevice = VirtualDevice.getInstance();

        virtualDevice.addBeatListener(beat -> {
            if (!beat.isTempoMaster()) return;
            onBeat(beat.getBeatWithinBar());
        });

        WaveformFinder.getInstance().addWaveformListener(new WaveformListener() {
            @Override
            public void previewChanged(WaveformPreviewUpdate update) {
                log.info(update.preview.toString());
            }

            @Override
            public void detailChanged(WaveformDetailUpdate update) {
                log.info(update.detail.toString());
            }
        });

        BeatGridFinder.getInstance().addBeatGridListener(new BeatGridListener() {
            @Override
            public void beatGridChanged(BeatGridUpdate update) {
                log.info(update.toString());
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
    public void modifyBpm(Modifier modifier, float value) {

    }
}
