
/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */

import com.kms.katalon.core.testobject.TestObject

import java.lang.String

import org.openqa.selenium.WebElement


def static "com.jira.components.JSelect.selectByText"(
    	TestObject container	
     , 	TestObject o	
     , 	String optionText	) {
    (new com.jira.components.JSelect()).selectByText(
        	container
         , 	o
         , 	optionText)
}

def static "com.jira.components.JSelect.selectByText"(
    	TestObject container	
     , 	String labelSelect	
     , 	String optionText	) {
    (new com.jira.components.JSelect()).selectByText(
        	container
         , 	labelSelect
         , 	optionText)
}

def static "com.jira.components.JSelect.selectByText"(
    	TestObject container	
     , 	WebElement el	
     , 	String optionText	) {
    (new com.jira.components.JSelect()).selectByText(
        	container
         , 	el
         , 	optionText)
}

def static "myPackage.myKeyword.refreshBrowser"() {
    (new myPackage.myKeyword()).refreshBrowser()
}

def static "myPackage.myKeyword.myKeywordPrint"(
    	String msg	) {
    (new myPackage.myKeyword()).myKeywordPrint(
        	msg)
}

def static "myPackage.myKeyword.clickElement"(
    	TestObject to	) {
    (new myPackage.myKeyword()).clickElement(
        	to)
}

def static "myPackage.myKeyword.getHtmlTableRows"(
    	TestObject table	
     , 	String outerTagName	) {
    (new myPackage.myKeyword()).getHtmlTableRows(
        	table
         , 	outerTagName)
}
