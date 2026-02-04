package com.FerdiStro.network.exceptions;

public class NetworkNotFoundException extends RuntimeException {
    public NetworkNotFoundException() {
        super("Network not ready for CDJ-paring. Network nor found. Name: en0");
    }
}
