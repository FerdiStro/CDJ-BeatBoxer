package com.FerdiStro.network;

import com.FerdiStro.LogUtils;
import lombok.Getter;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.deepsymmetry.beatlink.DeviceAnnouncement;
import org.deepsymmetry.beatlink.DeviceFinder;

import java.net.SocketException;
import java.util.Set;

import static java.lang.Thread.sleep;

public class Finder {

    protected static final Logger log = LogManager.getLogger();
    private static Finder INSTANCE = null;
    private final Integer maxAttempts;
    @Getter
    private Set<DeviceAnnouncement> deviceAnnouncements;

    private Finder(Integer maxAttempts) {
        this.maxAttempts = maxAttempts;
        this.refreshDeviceList();
    }

    public static Finder getInstance(Integer maxAttempts) {
        if (INSTANCE == null) {
            INSTANCE = new Finder(maxAttempts);
        }
        return INSTANCE;
    }

    public void refreshDeviceList() {
        DeviceFinder deviceFinder = DeviceFinder.getInstance();

        boolean deviceFound = false;
        int cdjAttempts = 0;


        while (cdjAttempts < this.maxAttempts && !deviceFound) {
            log.info(LogUtils.LINE_SEPARATOR);
            log.info("Attempt to find Device. Attempt: " + cdjAttempts);
            try {
                deviceFinder.start();
                if (!deviceFinder.getCurrentDevices().isEmpty()) {
                    deviceFound = true;
                    this.deviceAnnouncements = deviceFinder.getCurrentDevices();
                    log.info("Devices found:");
                    log.info(LogUtils.LINE_SEPARATOR);
                    assert deviceAnnouncements != null;
                    for (DeviceAnnouncement device : deviceAnnouncements) {
                        log.info("Number:  {}", device.getDeviceNumber());
                        log.info("Name:    {}", device.getDeviceName());
                        log.info("Address: {}", device.getAddress());
                        log.info("---");
                    }
                    break;
                }
                cdjAttempts++;
                log.info("Not found, retry again");
                sleep(5000);
            } catch (SocketException | InterruptedException e) {
                log.error(e.toString());
            }
        }

    }

}
