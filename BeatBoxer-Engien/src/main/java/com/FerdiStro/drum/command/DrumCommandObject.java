package com.FerdiStro.drum.command;


import lombok.Getter;

public class DrumCommandObject {
    @Getter
    private final DrumCommand command;

    private final String message;

    @Getter
    private int beatPosition;
    @Getter
    private String fileName;

    public DrumCommandObject(DrumCommand command, int beatPosition, String filePath) {
        if (command != DrumCommand.ADD_SOUND && command != DrumCommand.REMOVE_SOUND) {
            this.message = "Wrong constructor. Command should be ignored";
            this.command = DrumCommand.IGNORE;
            return;
        }
        this.command = command;
        this.message = "Add/Remove sound (Command: " + command + ") with filePath: " + filePath + " to beat " + beatPosition;
        this.beatPosition = beatPosition;
        this.fileName = filePath;
    }


    public String toString() {
        return message;
    }


}
