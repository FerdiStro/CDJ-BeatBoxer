package com.FerdiStro.drum.modes.pattern.objects;

import java.util.List;
import java.util.Objects;
import java.util.UUID;


public record PatternMetaData(Long patternId, String name, List<PatternMetaDataModes> modes,
                              List<String> tags) {
    public PatternMetaData {
        Objects.requireNonNull(patternId);
        Objects.requireNonNull(name);
        Objects.requireNonNull(modes);
    }
}