import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://client-gcms-br.int.thomsonreuters.com/GCMSWeb/sso/logon.do')

WebUI.setText(findTestObject('Object Repository/GCMS1.0 NEW/Page_Bem-vindo  aplicao de WIP/input_ID Usurio_username'), 'X011352')

WebUI.setEncryptedText(findTestObject('Object Repository/GCMS1.0/Page_Bem-vindo  aplicao de WIP/input_Senha_password'), 
    'Rh2adoAtdIo/Ypo/nzlFog==')

WebUI.selectOptionByValue(findTestObject('Object Repository/GCMS1.0/Page_Bem-vindo  aplicao de WIP/select_Portugus English Espaol'), 
    'pt', true)

WebUI.click(findTestObject('GCMS1.0/Page_Bem-vindo  aplicao de WIP/select_Portugus English Espaol'))

WebUI.click(findTestObject('GCMS1.0/Page_Bem-vindo  aplicao de WIP/English'))

WebUI.click(findTestObject('Object Repository/GCMS1.0 NEW/Page_Welcome to WIP module/input_Language_buttonInform'))

WebUI.click(findTestObject('Object Repository/GCMS1.0 NEW/Page_GCMS WIP/a_BIBLIOGRAPHY'))

WebUI.click(findTestObject('Object Repository/GCMS1.0 NEW/Page_GCMS WIP/a_Bibliographic documents'))

WebUI.click(findTestObject('Object Repository/GCMS1.0 NEW/Page_GCMS WIP/a_Search'))

WebUI.click(findTestObject('Object Repository/GCMS1.0 NEW/Page_GCMS Search/input_Load_2'))

WebUI.click(findTestObject('Object Repository/GCMS1.0 NEW/Page_Thomson Reuters/a_DTR199219'))

WebUI.click(findTestObject('Object Repository/GCMS1.0 NEW/Page_Document detail/input_FormatHTMLRTFPDFXML_2'))

WebUI.delay(5)

WebUI.click(findTestObject('Object Repository/GCMS1.0 NEW/Page_Document Edition/input_FormatHTMLRTFPDFXML_2'))

WebUI.delay(5)

//WebUI.click(findTestObject('Object Repository/GCMS1.0 NEW/Page_Document Edition/td_WIP'))
WebUI.click(findTestObject('GCMS1.0 NEW/Page_Document Edition/a_WIP'))

WebUI.click(findTestObject('Object Repository/GCMS1.0 NEW/Page_WIP/a_BIBLIOGRAPHY'))

WebUI.click(findTestObject('Object Repository/GCMS1.0 NEW/Page_WIP/a_Logoff'))

