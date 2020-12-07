<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Daftar Product</name>
   <tag></tag>
   <elementGuidId>5b39cc43-986c-44b5-b8fc-65523ee40910</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;[\n  {\n    \&quot;product_id\&quot;: \&quot;1\&quot;,\n    \&quot;parent_id\&quot;: \&quot;0\&quot;,\n    \&quot;product_name\&quot;: \&quot;Pulsa Elektrik\&quot;,\n    \&quot;product\&quot;: \&quot;\&quot;,\n    \&quot;code\&quot;: \&quot;\&quot;,\n    \&quot;prefix\&quot;: \&quot;\&quot;,\n    \&quot;description\&quot;: \&quot;Transaksi Untuk Pulsa\&quot;,\n    \&quot;is_active\&quot;: \&quot;1\&quot;\n  },\n  {\n    \&quot;product_id\&quot;: \&quot;2\&quot;,\n    \&quot;parent_id\&quot;: \&quot;0\&quot;,\n    \&quot;product_name\&quot;: \&quot;TOKEN PLN\&quot;,\n    \&quot;product\&quot;: \&quot;\&quot;,\n    \&quot;code\&quot;: \&quot;\&quot;,\n    \&quot;prefix\&quot;: \&quot;\&quot;,\n    \&quot;description\&quot;: \&quot;Transaksi untuk Voucher pln prabayar\&quot;,\n    \&quot;is_active\&quot;: \&quot;1\&quot;\n  },\n  {\n    \&quot;product_id\&quot;: \&quot;12\&quot;,\n    \&quot;parent_id\&quot;: \&quot;0\&quot;,\n    \&quot;product_name\&quot;: \&quot;Voucher Game\&quot;,\n    \&quot;product\&quot;: \&quot;\&quot;,\n    \&quot;code\&quot;: \&quot;\&quot;,\n    \&quot;prefix\&quot;: \&quot;\&quot;,\n    \&quot;description\&quot;: \&quot;I\&quot;,\n    \&quot;is_active\&quot;: \&quot;1\&quot;\n  },\n  {\n    \&quot;product_id\&quot;: \&quot;84\&quot;,\n    \&quot;parent_id\&quot;: \&quot;0\&quot;,\n    \&quot;product_name\&quot;: \&quot;Pulsa Internet\&quot;,\n    \&quot;product\&quot;: \&quot;\&quot;,\n    \&quot;code\&quot;: \&quot;\&quot;,\n    \&quot;prefix\&quot;: \&quot;\&quot;,\n    \&quot;description\&quot;: \&quot;\&quot;,\n    \&quot;is_active\&quot;: \&quot;1\&quot;\n  },\n  {\n    \&quot;product_id\&quot;: \&quot;87\&quot;,\n    \&quot;parent_id\&quot;: \&quot;0\&quot;,\n    \&quot;product_name\&quot;: \&quot;Gift Card\&quot;,\n    \&quot;product\&quot;: \&quot;\&quot;,\n    \&quot;code\&quot;: \&quot;\&quot;,\n    \&quot;prefix\&quot;: \&quot;\&quot;,\n    \&quot;description\&quot;: \&quot;\&quot;,\n    \&quot;is_active\&quot;: \&quot;1\&quot;\n  }\n]&quot;,
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
   <restUrl>https://private-anon-f3443efa7f-bisatopup.apiary-mock.com/product</restUrl>
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
