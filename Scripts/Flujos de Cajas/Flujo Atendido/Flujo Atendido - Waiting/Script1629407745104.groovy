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
import com.kms.katalon.core.logging.KeywordLogger as KeywordLogger

'Request a resolver qr'
response = WS.sendRequest(findTestObject('Resolve Qr', [('callerId') : GlobalVariable.mlaTestUserId, ('scope') : GlobalVariable.scope
            , ('siteId') : 'MLA', ('qrData') : '00020101021143520016com.mercadolibre0128https://mpago.la/pos/1043915501300091234567895204970053030325802AR5909FULL NAME6010CITY LEGAL63040AAB']))

'Print de response'
new KeywordLogger().logInfo(response.getResponseText())

'Verificación status code'
WS.verifyResponseStatusCode(response, 200)

'Verificación id de pantalla'
WS.verifyElementPropertyValue(response, 'id', 'generic_animated_waiting')

'Verificación texto en pantalla'
WS.verifyElementPropertyValue(response, 'screens_info.screens[0].title', 'Avisale al cajero que querés pagar con la app de Mercado Pago')

