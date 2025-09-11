function doGet(e) {
}

function doPost(e) {
    var data = JSON.parse(e.postData.contents);
    switch (e.parameter.action) {
        case "createSheet":
            var spreadsheetId = data.spreadsheetId, sheetName = data.sheetName;
            var spreadsheet = SpreadsheetApp.openById(spreadsheetId);
            var sourceSheet = spreadsheet.getSheetByName("Example");
            var newSheet = sourceSheet.copyTo(spreadsheet);
            newSheet.setName(sheetName);
            var duplicatedSheet = spreadsheet.getSheetByName("Example (копия)"); // i feel myself like a total idiot, but somehow this happens
            if (duplicatedSheet != undefined) {
                spreadsheet.deleteSheet(duplicatedSheet);
            }
            return ContentService.createTextOutput(JSON.stringify({ created_sheet_id: newSheet.getSheetId() })).setMimeType(ContentService.MimeType.JSON);
        default:
            return ContentService.createTextOutput("Something else was requested").setMimeType(ContentService.MimeType.JSON);
    }
}
