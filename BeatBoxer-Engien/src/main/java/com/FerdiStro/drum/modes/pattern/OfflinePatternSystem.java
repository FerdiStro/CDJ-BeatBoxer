package com.FerdiStro.drum.modes.pattern;

import com.FerdiStro.drum.modes.pattern.objects.PatternMetaData;
import com.FerdiStro.drum.modes.pattern.objects.PatternMetaDataModes;
import com.FerdiStro.drum.modes.pattern.ques.SearchQue;

import java.util.List;

public class OfflinePatternSystem extends AbstractPatternSystem {
    private static final String pathFile = "";

    @Override
    PatternMetaDataModes getPatternMetaDataMode() {
        return PatternMetaDataModes.OFFLINE;
    }

    @Override
    List<PatternMetaData> specificSearchPattern(SearchQue searchQue) {
        return List.of();
    }


}
