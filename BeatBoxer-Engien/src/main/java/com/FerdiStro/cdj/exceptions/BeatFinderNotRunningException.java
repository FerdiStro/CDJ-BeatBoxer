package com.FerdiStro.cdj.exceptions;

public class BeatFinderNotRunningException extends RuntimeException {
    public BeatFinderNotRunningException(String virtualDevice) {
        super("BeatFinder Not Running! on VirtualDive: " + virtualDevice);
    }
}
