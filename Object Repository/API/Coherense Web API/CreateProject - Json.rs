<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateProject - Json</name>
   <tag></tag>
   <elementGuidId>b0c5b83b-b57f-4b63-b20c-7298b6658ee4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;properties\&quot;: {\n    \&quot;description\&quot;: \&quot;Teste5\&quot;,\n    \&quot;itemType\&quot;: \&quot;DefinitionProject\&quot;,\n    \&quot;name\&quot;: \&quot;Teste5\&quot;,\n    \&quot;namespace\&quot;: \&quot;teste5\&quot;\n  },\n  \&quot;uuids\&quot;: [\n    \&quot;13b8f0bb-2e83-4cfb-a07e-f174e5f7ddcb\&quot;\n  ],\n  \&quot;displayFields\&quot;: [\n    \&quot;name\&quot;, \&quot;namespace\&quot;, \&quot;description\&quot;, \&quot;revision\&quot;, \&quot;owner\&quot;, \&quot;currentUser\&quot;, \&quot;tagVersion\&quot;, \&quot;itemType\&quot;, \&quot;status\&quot;, \&quot;creationDate\&quot;, \&quot;lastModificationDate\&quot; \n  ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://localhost:8080/ProjectManagement/CreateProject</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
