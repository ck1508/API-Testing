<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetByName</name>
   <tag></tag>
   <elementGuidId>1f6e657d-4cbb-45a8-8772-ce04227a9df7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>SOAPAction</name>
      <type>Main</type>
      <value>http://tempuri.org/SOAP.Demo.GetByName</value>
      <webElementGuid>ae3f9055-a6e0-4fad-9f52-7f8df6f555d5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
      <webElementGuid>0ecd7e2c-fa29-4a8a-bf6e-1da154ed6514</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tem=&quot;http://tempuri.org&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;tem:GetByName>
         &lt;tem:name>gero et&lt;/tem:name>
      &lt;/tem:GetByName>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>https://www.crcind.com:443/csp/samples/SOAP.Demo.cls</soapServiceEndpoint>
   <soapServiceFunction>GetByName</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>https://www.crcind.com/csp/samples/SOAP.Demo.CLS?WSDL=1</wsdlAddress>
</WebServiceRequestEntity>
