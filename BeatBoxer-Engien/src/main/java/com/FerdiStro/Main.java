package com.FerdiStro;

import com.FerdiStro.cdj.modes.AbstractMode;
import com.FerdiStro.cdj.modes.ConnectMode;
import com.FerdiStro.cdj.modes.OfflineMode;
import com.FerdiStro.network.Finder;
import com.FerdiStro.network.NetWorkInfo;
import com.FerdiStro.network.exceptions.NetworkNotFoundException;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.deepsymmetry.beatlink.DeviceAnnouncement;

import java.util.Set;


public class Main {


    protected static final Logger log = LogManager.getLogger();


    private static final int MAX_ATTEMPTS = 10;

    public static void main(String[] args) {
        log.info(LogUtils.LINE_SEPARATOR);
        log.info(LogUtils.HEADER);
        log.info(LogUtils.LINE_SEPARATOR);

        NetWorkInfo netWorkInfo = NetWorkInfo.getInstance();

        if (!netWorkInfo.status()) {
            throw new NetworkNotFoundException();
        }

        Finder finder = Finder.getInstance(MAX_ATTEMPTS);
        Set<DeviceAnnouncement> deviceAnnouncements = finder.getDeviceAnnouncements();


        AbstractMode beatMode = null;
        if (deviceAnnouncements.isEmpty()) {
            beatMode = new OfflineMode();
        } else {
            beatMode = new ConnectMode();
        }

        beatMode.startUp();
        beatMode.printAnalytics();




    }
}