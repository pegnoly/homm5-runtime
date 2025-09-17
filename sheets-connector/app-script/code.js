function doGet(e) {
}

function doPost(e) {
  var data = JSON.parse(e.postData.contents);
  switch (e.parameter.action) {
    case "createSheet":
      try {
        var spreadsheetId = data.spreadsheetId, sheetName = data.sheetName;
        var spreadsheet = SpreadsheetApp.openById(spreadsheetId);
        var sourceSheet = spreadsheet.getSheetByName("Example");
        var sourceRange = sourceSheet.getRange("A1:H24");
        var newSheet = spreadsheet.insertSheet();
        var generatedName = newSheet.getName();
        var updatedSheet = newSheet.setName(sheetName);
        var duplicatedSheet = spreadsheet.getSheetByName(generatedName); // i feel myself like a total idiot, but somehow this happens
        if (duplicatedSheet != undefined) {
          spreadsheet.deleteSheet(duplicatedSheet);
        }
        const destinationRange = updatedSheet.getRange("A1:H24");
        sourceRange.copyTo(destinationRange);
        for (let col = 1; col <= 8; col++) {
            const width = sourceSheet.getColumnWidth(col);
            updatedSheet.setColumnWidth(col, width);
        }
        return ContentService.createTextOutput(JSON.stringify({ created_sheet_id: newSheet.getSheetId() })).setMimeType(ContentService.MimeType.JSON);
      }
      catch(error) {
        return ContentService.createTextOutput(JSON.stringify({ error: error.toString()  })).setMimeType(ContentService.MimeType.JSON);
      }
    case "addArtifactsData":
      try {
        var spreadsheetId = data.spreadsheetId, sheetId = data.sheetId;
        var spreadsheet = SpreadsheetApp.openById(spreadsheetId);
        var sourceSheet = spreadsheet.getSheetByName("Example");
        var destinationSheet = spreadsheet.getSheetById(sheetId);
        var sourceRange = sourceSheet.getRange("A27:H32");
        var destinationRange = destinationSheet.getRange("A27:H32");
        sourceRange.copyTo(destinationRange);
        return ContentService.createTextOutput("Sheet was updated with arts data").setMimeType(ContentService.MimeType.JSON);
      }
      catch(error) {
        return ContentService.createTextOutput(JSON.stringify({ error: error.toString()  })).setMimeType(ContentService.MimeType.JSON);
      }
    default:
      return ContentService.createTextOutput("Something else was requested").setMimeType(ContentService.MimeType.JSON);
  }
}
