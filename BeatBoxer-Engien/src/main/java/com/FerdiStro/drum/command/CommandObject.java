package com.FerdiStro.drum.command;


import lombok.Getter;

public class CommandObject {
    @Getter
    private final DrumCommand command;

    private final String message;

    @Getter
    private int beatPosition;
    @Getter
    private String fileName;

    public CommandObject(DrumCommand command, int beatPosition, String filePath) {
        if (command != DrumCommand.ADD_SOUND) {
            this.message = "Wrong constructor. Command should be ignored";
            this.command = DrumCommand.IGNORE;
            return;
        }
        this.command = command;
        this.message = "Add sound (Command: " + command + ") with filePath" + filePath + " to beat " + beatPosition;
        this.beatPosition = beatPosition;
        this.fileName = filePath;
    }


    public String toString() {
        return message;
    }



}
