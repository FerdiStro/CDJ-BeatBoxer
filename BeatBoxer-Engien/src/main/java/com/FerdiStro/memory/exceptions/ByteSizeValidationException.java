package com.FerdiStro.memory.exceptions;

public class ByteSizeValidationException extends RuntimeException {
    public ByteSizeValidationException(String valueName, byte expectedSize, byte actualSize) {
        super("Error on validation value: " + valueName + ". Expected Size: " + expectedSize + ". ActualSize: " + actualSize);
    }
}
