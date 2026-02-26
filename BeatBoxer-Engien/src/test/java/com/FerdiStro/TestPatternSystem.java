package com.FerdiStro;

import com.FerdiStro.drum.modes.pattern.OfflinePatternSystem;
import com.FerdiStro.drum.modes.pattern.objects.PatternMetaData;
import com.FerdiStro.drum.modes.pattern.ques.SearchQue;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestPatternSystem {

    @Test
    void testLoadSystem() throws InterruptedException {
        OfflinePatternSystem offlinePatternSystem = new OfflinePatternSystem();

        offlinePatternSystem.refreshCacheWithPath("src/test/resources/pattern_meta_data.json");

        offlinePatternSystem.setSearchQue(SearchQue.MODE);
        Thread thread = new Thread(offlinePatternSystem);

        //Simulate Thread running
        thread.start();
        Thread.sleep(100);
        offlinePatternSystem.stop();


        PatternMetaData[] searchedPatternMetaData = offlinePatternSystem.getSearchedPatternMetaData();

        Assertions.assertEquals(10, searchedPatternMetaData.length);
        Assertions.assertEquals(1, searchedPatternMetaData[0].patternId());
        Assertions.assertEquals(2, searchedPatternMetaData[1].patternId());


    }

}
