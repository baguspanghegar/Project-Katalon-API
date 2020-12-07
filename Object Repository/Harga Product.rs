<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Harga Product</name>
   <tag></tag>
   <elementGuidId>1fc8e550-b6a1-4ece-b85e-ec89509fc9e4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;product_detail_id\&quot;: \&quot;43\&quot;,\n  \&quot;product_id\&quot;: \&quot;11\&quot;,\n  \&quot;product_code\&quot;: \&quot;A5\&quot;,\n  \&quot;name\&quot;: \&quot;AS 5000\&quot;,\n  \&quot;price\&quot;: \&quot;5800\&quot;,\n  \&quot;base_price\&quot;: \&quot;5600\&quot;,\n  \&quot;note\&quot;: \&quot;\&quot;,\n  \&quot;is_active\&quot;: \&quot;1\&quot;,\n  \&quot;price_text\&quot;: \&quot;Rp. 5,600\&quot;\n}&quot;,
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
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://private-anon-f3443efa7f-bisatopup.apiary-mock.com/harga/product/:id</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
