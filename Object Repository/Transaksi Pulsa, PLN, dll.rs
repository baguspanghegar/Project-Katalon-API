<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Transaksi Pulsa, PLN, dll</name>
   <tag></tag>
   <elementGuidId>da9e56c4-8620-4f67-ba6b-877a367c4a7b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;error\&quot;: false,\n  \&quot;message\&quot;: \&quot;Success\&quot;,\n  \&quot;trans_id\&quot;: 1382153,\n  \&quot;transaksi\&quot;: {\n    \&quot;trans_id\&quot;: 1382153,\n    \&quot;tanggal\&quot;: \&quot;10 Jan 2018\&quot;,\n    \&quot;time\&quot;: \&quot;21:58:09\&quot;,\n    \&quot;product\&quot;: \&quot;TELKOMSEL 5.000\&quot;,\n    \&quot;product_id\&quot;: 11,\n    \&quot;admin_fee\&quot;: null,\n    \&quot;product_name\&quot;: \&quot;TELKOMSEL\&quot;,\n    \&quot;product_detail\&quot;: \&quot;TELKOMSEL 5.000\&quot;,\n    \&quot;harga\&quot;: 5755,\n    \&quot;base_price\&quot;: 5755,\n    \&quot;payment\&quot;: \&quot;E-Wallet\&quot;,\n    \&quot;no_pelanggan\&quot;: \&quot;082364056730\&quot;,\n    \&quot;no_hp_pelanggan\&quot;: null,\n    \&quot;kode_unik\&quot;: null,\n    \&quot;expired_date\&quot;: null,\n    \&quot;note\&quot;: null,\n    \&quot;token\&quot;: null,\n    \&quot;status\&quot;: \&quot;Terbayar\&quot;,\n    \&quot;status_id\&quot;: 3,\n    \&quot;pembayaran\&quot;: {\n      \&quot;bank\&quot;: \&quot;E-Wallet\&quot;,\n      \&quot;rekening\&quot;: \&quot;\&quot;,\n      \&quot;name\&quot;: \&quot;\&quot;,\n      \&quot;payment_id\&quot;: 13\n    }\n  }\n}&quot;,
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
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://private-anon-f3443efa7f-bisatopup.apiary-mock.com/transaksi/beli</restUrl>
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
