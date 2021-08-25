<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Post V1 In Store Order</name>
   <tag></tag>
   <elementGuidId>1ddfc322-fefb-43df-91ea-cd3b45360a45</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;items\&quot; :[{\n\&quot;title\&quot; : \&quot;Papitas \&quot;,\n\&quot;description\&quot; : \&quot;Lays\&quot;,\n \t\&quot;currency_id\&quot; : \&quot;ARS\&quot;,\n \t\&quot;unit_price\&quot; : 8.0,\n  \t\&quot;quantity\&quot; : 1\n \t}]\n}&quot;,
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
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${baseUrl}/mpmobile/${scope}/instore/qr/${posId}?client.id=1505&amp;caller.siteId=${siteId}&amp;caller.id=${callerId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6f2e8699-53c3-4110-bef1-ed19d44b0a31</id>
      <masked>false</masked>
      <name>siteId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7bc3724d-3cfd-472e-b2f0-2e55e1f72e7d</id>
      <masked>false</masked>
      <name>callerId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.baseUrl</defaultValue>
      <description></description>
      <id>0cc936a7-8c42-4925-a05e-af644f63cca1</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ordersScope</defaultValue>
      <description></description>
      <id>04982c9b-2fa5-44fb-bf1b-9c28da848c0f</id>
      <masked>false</masked>
      <name>scope</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>fb6ed88e-9c4f-4b47-be72-4556988cbbee</id>
      <masked>false</masked>
      <name>posId</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
