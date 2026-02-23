package com.FerdiStro.drum.modes;


import com.FerdiStro.cdj.VirtualDevice;
import com.FerdiStro.memory.WaveformSharedMemory;
import com.FerdiStro.memory.bus.MemoryUpdateCommand;
import org.deepsymmetry.beatlink.Beat;
import org.deepsymmetry.beatlink.DeviceUpdate;
import org.deepsymmetry.beatlink.MasterListener;
import org.deepsymmetry.beatlink.data.*;

import java.nio.ByteBuffer;


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

    private void updateTrackPosition(int playerId, TrackPositionUpdate update) {
        if (update != null) {
            WaveformSharedMemory waveformSharedMemories = memoryProvider.getWaveformSharedMemories(playerId);

            long playHeadMs = update.milliseconds;
            int beatNumber = update.beatNumber;
            int bpm = update.beatGrid.getBpm(beatNumber);

            waveformSharedMemories.updatePlayHead(playHeadMs, bpm);
        }

    }

    @Override
    public void startUp() {
        VirtualDevice virtualDevice = VirtualDevice.getInstance();

        //BeatGrid Listeners
        virtualDevice.addBeatGridListener(gridUpdate -> {
            int playerId = gridUpdate.player;
            WaveformSharedMemory waveformSharedMemories = memoryProvider.getWaveformSharedMemories(playerId);

            //Meta-Data
            long playHeadMs = TimeFinder.getInstance().getTimeFor(playerId);
            int bpm = gridUpdate.beatGrid.getBpm(playerId);
            waveformSharedMemories.updatePlayHead(playHeadMs, bpm);


            //Grid as RawBytes
            ByteBuffer rawData = gridUpdate.beatGrid.getRawData();
            waveformSharedMemories.updateBeatGrid(rawData);
        });

        //TrackPosition listeners only support 2 cdjs with player number 1 & 2
        virtualDevice.addTimeFinders((TrackPositionUpdate update) -> updateTrackPosition(1, update), update -> updateTrackPosition(2, update));

        virtualDevice.addWaveFromListener(new WaveformListener() {
            @Override
            public void previewChanged(WaveformPreviewUpdate update) {
                //ignore
            }

            @Override
            public void detailChanged(WaveformDetailUpdate update) {
                WaveformSharedMemory waveformSharedMemories = memoryProvider.getWaveformSharedMemories(update.player);
                waveformSharedMemories.updateWaveFrom(update.detail);
            }
        });

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



