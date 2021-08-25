<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Post V0 Preference</name>
   <tag></tag>
   <elementGuidId>97b8b835-71af-4255-81d3-fddc440d2e3d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;additional_info\&quot;: \&quot;instore-qr\&quot;,\n    \&quot;branch_id\&quot;: \&quot;test\&quot;,\n    \&quot;collector_id\&quot;: 515488630,\n    \&quot;expires\&quot;: false,\n    \&quot;external_reference\&quot;: \&quot;test\&quot;,\n    \&quot;items\&quot;: [\n        {\n            \&quot;quantity\&quot;: 1,\n            \&quot;title\&quot;: \&quot;Producto de Test\&quot;,\n            \&quot;unit_price\&quot;: 10\n        }\n    ],\n    \&quot;operation_type\&quot;: \&quot;regular_payment\&quot;,\n    \&quot;payment_methods\&quot;: {\n        \&quot;excluded_payment_methods\&quot;: [\n            {\n                \&quot;id\&quot;: \&quot;redlink\&quot;\n            },\n            {\n                \&quot;id\&quot;: \&quot;pagofacil\&quot;\n            },\n            {\n                \&quot;id\&quot;: \&quot;bapropagos\&quot;\n            },\n            {\n                \&quot;id\&quot;: \&quot;rapipago\&quot;\n            },\n            {\n                \&quot;id\&quot;: \&quot;cargavirtual\&quot;\n            }\n        ],\n        \&quot;excluded_payment_types\&quot;: [\n            {\n                \&quot;id\&quot;: \&quot;ticket\&quot;\n            },\n            {\n                \&quot;id\&quot;: \&quot;atm\&quot;\n            },\n            {\n                \&quot;id\&quot;: \&quot;digital_currency\&quot;\n            }\n        ]\n    },\n    \&quot;processing_modes\&quot;: [\n        \&quot;aggregator\&quot;\n    ]\n}\n&quot;,
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
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${baseUrl}/checkout/preferences?caller.id=515488630&amp;client.id=1505&amp;caller.siteId=MLA</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
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
