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

WebUI.navigateToUrl('https://secure.qa-webbuildercs.com/emoadmin/index.jsp')

WebUI.setText(findTestObject('Object Repository/Page_Login to Web Builder CS/input_Username_j_username'), 'tm_admin')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Login to Web Builder CS/input_Password_j_password'), '9VbJ3BRDhQ7s6QwO4THcWA==')

WebUI.click(findTestObject('Object Repository/Page_Login to Web Builder CS/input_Forgot your password_loginSubmit'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_qa Aspartame by Thomson Reuters/select_Choose an Action.Show AssignmentsOve_f2a726'), 
    'show_special', true)

WebUI.click(findTestObject('Object Repository/Page_qa Aspartame by Thomson Reuters/div_Unique Test Data'))
WebUI.delay(5)


WebUI.click(findTestObject('Object Repository/Page_qa Aspartame by Thomson Reuters/a_Manage Site Wizard'))
WebUI.delay(5)

WebUI.switchToWindowTitle('[qa] Dashboard - Web Builder CS')


WebUI.waitForElementClickable(findTestObject('Object Repository/Page_qa Dashboard - Web Builder CS/a_Edit Site'), 10)

WebUI.click(findTestObject('Object Repository/Page_qa Dashboard - Web Builder CS/a_Edit Site'))

WebUI.click(findTestObject('Object Repository/Page_qa Site Wizard - Web Builder CS/a_Newsletter'))

WebUI.click(findTestObject('Object Repository/Page_qa Newsletter - Web Builder CS/a_Settings'))

WebUI.closeBrowser()

