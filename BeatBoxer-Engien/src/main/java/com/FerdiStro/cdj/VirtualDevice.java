package com.FerdiStro.cdj;

import com.FerdiStro.LogUtils;
import com.FerdiStro.cdj.exceptions.BeatFinderNotRunningException;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.deepsymmetry.beatlink.BeatFinder;
import org.deepsymmetry.beatlink.BeatListener;
import org.deepsymmetry.beatlink.VirtualCdj;

public class VirtualDevice {


    protected static final Logger log = LogManager.getLogger();
    private static final  Byte DEVICE_NUMBER = 4;

    private static  VirtualDevice  INSTANCE = null;


    private VirtualDevice(){
        log.info(LogUtils.LINE_SEPARATOR);
        log.info("Starting Virtual DJ on Device Number  {}",DEVICE_NUMBER);
        VirtualCdj cdj = VirtualCdj.getInstance();
        cdj.setDeviceNumber(DEVICE_NUMBER);

        try {
            cdj.start();

        } catch (Exception e) {
            log.error(e.toString());
            throw new RuntimeException(e);
        }

        cdj.setSynced(true);
        cdj.setPlaying(true);

        BeatFinder beatFinder = BeatFinder.getInstance();

        if(!beatFinder.isRunning()){
            throw new BeatFinderNotRunningException(cdj.toString());
        }
    }

    public void addBeatListener(BeatListener beatListener){

    }


    public static VirtualDevice getInstance() {
        if(INSTANCE == null){
            INSTANCE = new VirtualDevice();
        }
        return INSTANCE;
    }


}
