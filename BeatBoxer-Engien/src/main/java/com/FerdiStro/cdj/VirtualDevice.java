package com.FerdiStro.cdj;

import com.FerdiStro.LogUtils;
import com.FerdiStro.cdj.exceptions.BeatFinderNotRunningException;
import com.FerdiStro.cdj.exceptions.BecomeMasterException;
import com.FerdiStro.cdj.modes.ConnectMode;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.deepsymmetry.beatlink.BeatFinder;
import org.deepsymmetry.beatlink.DeviceUpdate;
import org.deepsymmetry.beatlink.MasterListener;
import org.deepsymmetry.beatlink.VirtualCdj;
import org.deepsymmetry.beatlink.data.ArtFinder;
import org.deepsymmetry.beatlink.data.BeatGridFinder;
import org.deepsymmetry.beatlink.data.MetadataFinder;
import org.deepsymmetry.beatlink.data.WaveformFinder;

import java.io.IOException;

public class VirtualDevice {


    protected static final Logger log = LogManager.getLogger();
    private static final Byte DEVICE_NUMBER = 4;

    private static VirtualDevice INSTANCE = null;

    private final VirtualCdj virtualCdj;


    private VirtualDevice() {
        log.info(LogUtils.LINE_SEPARATOR);
        log.info("Starting Virtual DJ on Device Number  {}", DEVICE_NUMBER);

        VirtualCdj cdj = VirtualCdj.getInstance();
        cdj.setDeviceNumber((byte) 4);


        try {
            cdj.start();

        } catch (Exception e) {
            log.error(e.toString());
            throw new RuntimeException(e);
        }

        cdj.setSynced(true);
        cdj.setPlaying(true);


        log.info("Enable sending status to other CDJs");
        try {
            cdj.setSendingStatus(true);
        } catch (IOException e) {
            log.error(e.toString());
            throw new RuntimeException(e);
        }


        log.info("Start all finders");
        try {
            MetadataFinder.getInstance().start();
            ArtFinder.getInstance().start();
            WaveformFinder.getInstance().start();
            BeatGridFinder.getInstance().start();
            BeatFinder.getInstance().start();
        } catch (Exception e) {
            log.error(e.toString());
            throw new IllegalStateException();
        }

        if (!BeatFinder.getInstance().isRunning()) {
            throw new BeatFinderNotRunningException(cdj.toString());
        }
        log.info("VirtualDevice ready! All finder up!");

        virtualCdj = cdj;
    }

    public static VirtualDevice getInstance() {
        if (INSTANCE == null) {
            INSTANCE = new VirtualDevice();
        }
        return INSTANCE;
    }

    public boolean istMaster() {
        return virtualCdj.isTempoMaster();
    }

    public void becomeMaster() {
        try {
            virtualCdj.becomeTempoMaster();
        } catch (IOException e) {
            log.error(e.toString());
            throw new BecomeMasterException(e.toString());
        }
    }

    public void addBeatListener(MasterListener masterListener) {
        virtualCdj.addMasterListener(masterListener);
    }

    public double getMasterBpm() {
        return virtualCdj.getMasterTempo();
    }


}
