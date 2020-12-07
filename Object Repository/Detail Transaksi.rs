<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Detail Transaksi</name>
   <tag></tag>
   <elementGuidId>2fb96eec-3a6f-4795-8315-a596675b427c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;trans_id\&quot;: \&quot;5816\&quot;,\n  \&quot;order_num\&quot;: \&quot;90\&quot;,\n  \&quot;product_id\&quot;: \&quot;31\&quot;,\n  \&quot;product_name\&quot;: \&quot;PLN\&quot;,\n  \&quot;product_detail_id\&quot;: \&quot;115\&quot;,\n  \&quot;no_hp\&quot;: \&quot;087719888886\&quot;,\n  \&quot;token\&quot;: \&quot;\&quot;,\n  \&quot;no_pelanggan\&quot;: \&quot;112000162978\&quot;,\n  \&quot;product_detail\&quot;: \&quot;PLN\&quot;,\n  \&quot;harga\&quot;: \&quot;311305\&quot;,\n  \&quot;note\&quot;: \&quot;\&quot;,\n  \&quot;status_id\&quot;: \&quot;5\&quot;,\n  \&quot;status\&quot;: \&quot;Cancelled\&quot;,\n  \&quot;created_at\&quot;: \&quot;2016-07-20 06:11:01\&quot;\n}&quot;,
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
   <restUrl>https://private-anon-f3443efa7f-bisatopup.apiary-mock.com/transaksi/detail/:id</restUrl>
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
