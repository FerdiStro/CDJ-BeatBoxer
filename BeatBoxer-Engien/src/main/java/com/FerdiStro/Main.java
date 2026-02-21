package com.FerdiStro;

import com.FerdiStro.drum.modes.AbstractMode;
import com.FerdiStro.drum.modes.ConnectMode;
import com.FerdiStro.drum.modes.OfflineMode;
import com.FerdiStro.drum.DrumMachine;
import com.FerdiStro.memory.SharedMemoryProvider;
import com.FerdiStro.network.Finder;
import com.FerdiStro.network.NetWorkInfo;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.deepsymmetry.beatlink.DeviceAnnouncement;

import java.util.Set;


public class Main {


    protected static final Logger log = LogManager.getLogger();


    private static final int MAX_ATTEMPTS = 0;

    public static void main(String[] args) {
        log.info(LogUtils.LINE_SEPARATOR);
        log.info(LogUtils.HEADER);
        log.info(LogUtils.LINE_SEPARATOR);
        log.info("Java Working Directory: {}", System.getProperty("user.dir"));

        SharedMemoryProvider sharedMemoryProvider = SharedMemoryProvider.getInstance();
        sharedMemoryProvider.start();

        NetWorkInfo netWorkInfo = NetWorkInfo.getInstance();

//        if (!netWorkInfo.status()) {
//            throw new NetworkNotFoundException();
//        }

        Finder finder = Finder.getInstance(MAX_ATTEMPTS);
        Set<DeviceAnnouncement> deviceAnnouncements = finder.getDeviceAnnouncements();


        AbstractMode beatMode = null;
        if (deviceAnnouncements == null || deviceAnnouncements.isEmpty()) {
            beatMode = new OfflineMode();
        } else {
            beatMode = new ConnectMode();
        }

        beatMode.startUp();
        beatMode.printAnalytics();

        DrumMachine drumMachine = new DrumMachine(beatMode);
        beatMode.setDrumMachineCommandLine(drumMachine);


        sharedMemoryProvider.setStopReading(false);


        log.info("BeatBoxer-Engine is ready. Can Start UI (UI_LISTENER_COMMAND: BACKEND_READY)");
    }
}