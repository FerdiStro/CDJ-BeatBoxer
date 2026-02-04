package com.FerdiStro.network;

import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import java.net.NetworkInterface;
import java.net.SocketException;
import java.util.Enumeration;

public class NetWorkInfo {
    protected static final Logger log = LogManager.getLogger();

    private static NetWorkInfo INSTANCE = null;

    private NetWorkInfo() {
        System.setProperty("java.net.preferIPv4Stack", "true");
        this.scan();
    }

    /**
     * @return boolean. Is true when en0 (lan) has multicast (needed for CDJ-connection)
     */
    public boolean status() {
        return this.status;
    }

    private boolean status = false;

    public static NetWorkInfo getInstance() {
        if (INSTANCE == null) INSTANCE = new NetWorkInfo();
        return INSTANCE;
    }


    public void scan() {
        try {
            Enumeration<NetworkInterface> interfaces = NetworkInterface.getNetworkInterfaces();
            while (interfaces.hasMoreElements()) {
                NetworkInterface ni = interfaces.nextElement();
                if (ni.isUp() && !ni.isLoopback()) {
                    if (ni.getName().equals("eth0") && ni.supportsMulticast()) {
                        this.status = true;
                    }
                    log.info("Interface: {}", ni.getName());
                    log.info("  - Supports Multicast: {}", ni.supportsMulticast());
                    log.info("  - Is PointToPoint: {}", ni.isPointToPoint());
                }
            }

        } catch (SocketException e) {
            log.error("Error while scanning network");
        }
    }
}
