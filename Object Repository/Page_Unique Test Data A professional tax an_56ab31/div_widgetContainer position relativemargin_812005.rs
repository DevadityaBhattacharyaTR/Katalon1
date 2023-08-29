<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_widgetContainer position relativemargin_812005</name>
   <tag></tag>
   <elementGuidId>5730acd7-ec34-46a7-9f21-0d734a6aa5c3</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='precontain']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#precontain</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>b16351c9-835d-45fb-9cda-9781a5f6b2c0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>precontain</value>
      <webElementGuid>00a1398d-5568-4366-bc6c-8ea67acea455</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>













				


			
			











				


			
			







	.widgetContainer {		
		position: relative;
		margin-left: 0;
		margin-bottom: 0;
		margin-right: 0;
		margin-top: 0;
		padding-top: 0;
		padding-right: 0;
		padding-bottom: 0;
		padding-left: 0;
	}
	#about_us div.widgetContainer{
		width: 100%;
	}




							
								








				


			
			







		
			 
 
Call us today 
for a free consultation:
(111) 222-3333 

		







 
			    			













				


			
			





Unique Test Data













				


			
			








A Test Firm







	/* The #nav is for sw4 sites only. go back and check them and change this back or something? */
	/* #sidebar-nav ul is for layout1 only */
	nav ul li,#nav ul li {
		float: left;
		list-style-type: none;
		margin: 0;
		padding: 0;
	}

	nav ul li a,
	nav ul li a:active,
	#nav ul li a,
	#nav ul li a:active {
		display: block;
		text-decoration: none;
	}

	nav ul li a:hover,
	#nav ul li a:hover {
		text-decoration: none;
	}

	nav ul li ul li,
	#nav ul li ul li {
		float: none;	
	}
	
	nav ul ul,
	#nav ul ul,
	#sidebar-nav ul {
		display:none;
		list-style:none;
	}
	nav select#select-nav,
	#nav select#select-nav,
	#select-nav {
		display:none;
	}
	/* Temp oldschool-horiz fix */
	ul.tabs ul ul {
		padding-bottom: 2px;
	}
	ul.tabs ul {
		padding-bottom: 8px;
	}

		
		HomeAbout Our FirmServicesLinksClient LoginContactTax ToolsTax TipsIndividualBusinessFinancialTax RatesDue DatesFinancial ToolsRetention GuideIRS FormsNewsMonthly NewsDaily NewsFinancial GuidesBankingBusinessFinancialInsuranceLife EventsTaxes
















				


			
			











				


			
			







	.widgetContainer {		
		position: relative;
		margin-left: 0;
		margin-bottom: 0;
		margin-right: 0;
		margin-top: 0;
		padding-top: 0;
		padding-right: 0;
		padding-bottom: 0;
		padding-left: 0;
	}
	#about_us div.widgetContainer{
		width: 100%;
	}




							
								











				


			
			



			
				.s3sliderImage span { visibility: hidden; }
			
			
			
			
			
			
				
				
						
							
							
						
						
							
							
						
						
							
							
						
						
							
							
						
					
				
			
		
		
			jQuery(document).ready(function(){
				jQuery('#s3slider').s3Slider({
					timeOut: 4000
				});
			});
		
	
	
	
	
  	
  	
  	
 
			    			













				


			
			











				


			
			







	.widgetContainer {		
		position: relative;
		margin-left: 0;
		margin-bottom: 0;
		margin-right: 0;
		margin-top: 0;
		padding-top: 0;
		padding-right: 0;
		padding-bottom: 0;
		padding-left: 0;
	}
	#about_us div.widgetContainer{
		width: 100%;
	}




							
								








				


			
			




	#login_widget_submit { display: block; }

#onvioSignIn #login_widget_submit, #loginForm #login_widget_submit{
margin: 15px auto;
padding: 20px 25px 20px 25px;
border-radius: 5px;
width: 125px;
height: 50px;
font-size: 16px!important;
line-height: 10px!important;
float: none;
text-align: center!important;

}



    function resetLoginForm() { // submits form
        document.getElementById(&quot;loginForm&quot;).reset();
    }
    function btnLoginClickAction()
    {
        if (document.getElementById(&quot;loginForm&quot;)) {
            setTimeout(&quot;resetLoginForm()&quot;, 5000); // set timout 
       }
    }

			
			
				Client Login
				
					
						
						
					
						Login
					
					
			    
			



 
			    			
							
								








				


			
			




	
	
	
	
	
	

			
	            
	                    Subscribe to our Newsletter
									
	                    	                    
										
	                    
	                    
	                    Submit
	                    
	            
           
		
		

jQuery(document).ready(function(){
	jQuery.validate({
		validateOnBlur: true,
		form : '#newslettersignupwidget',
		onSuccess: function(){
			newsletterSignUp();			
			return false;
		}
	});	
	
});
	
function newsletterSignUp(){
	var email = jQuery(&quot;#965339 .newsletterSignupEmail&quot;).val();
	  jQuery(&quot;#965339 .emailCheckIt&quot;).val(email);
	  var name = jQuery(&quot;.newsletterSignupName&quot;).val();
  	  var siteId = &quot;14905&quot;;
  	  jQuery.post(&quot;utilities/add-to-mailinglist.jsp&quot;, { name: name, email: email, site_id:siteId }, function(data){
	  	var result = data.status;
		if(result != &quot;error&quot;){
			var message = &quot;&lt;h5 style='text-align:center;'>&quot; + &quot;Success!&quot; + &quot;&lt;/h5>&lt;p style='font-weight:normal;text-align:center;height:auto;float:none;'>&quot; + &quot;You have been signed up for the mailing list.&quot; + &quot;&lt;/p>&quot;
			jQuery(&quot;#965339 .newsletterSignupDiv&quot;).html(message);
		}
	 });
}
	
	

 
			    			














				


			
			











				


			
			







	.widgetContainer {		
		position: relative;
		margin-left: 0;
		margin-bottom: 0;
		margin-right: 0;
		margin-top: 0;
		padding-top: 0;
		padding-right: 0;
		padding-bottom: 0;
		padding-left: 0;
	}
	#about_us div.widgetContainer{
		width: 100%;
	}

















				


			
			







	             	Home
	             	
	             	About Our Firm
	             	
	             	Services
	             	
	             	Client Login
	             	
	             	Contact
	             	
	             	Glossary
	             	
	             	Links
	             	












				


			
			














				


			
			




	
		.social_link_anchor { width: 33px; height: 32px;display:inline-block; }
		.twitter_anchor { background: url(emoAssets/images/twitter.png) no-repeat; }
		.facebook_anchor { background: url(emoAssets/images/facebook.png) no-repeat; }
		.linkedin_anchor { background: url(emoAssets/images/linkedin.png) no-repeat; }
		.yelp_anchor { background: url(emoAssets/images/yelp.png) no-repeat; }
		#get_footer_social ul li { list-style: none;display: inline; padding: 5px; }
		#get_footer_social { display: block; text-align:center; width: 100%; margin: 5px auto; }
		.social_links_widget li { list-style:none; }
	

			
				
				
				
			




© 2023 Unique Test Data   All Rights Reserved.
 	Web Builder CS: Websites for Accountants






</value>
      <webElementGuid>5f6535a9-bc25-4f39-8307-384ca103515b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;precontain&quot;)</value>
      <webElementGuid>b1f029fa-7cb7-4f26-9c86-a2d3fe6f9466</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='precontain']</value>
      <webElementGuid>4b9dbb47-e90d-4792-a14d-df1bb510d3d9</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]</value>
      <webElementGuid>1f4b3f10-4546-4fcb-875f-651c84aabc46</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'precontain' and (text() = concat(&quot;













				


			
			











				


			
			







	.widgetContainer {		
		position: relative;
		margin-left: 0;
		margin-bottom: 0;
		margin-right: 0;
		margin-top: 0;
		padding-top: 0;
		padding-right: 0;
		padding-bottom: 0;
		padding-left: 0;
	}
	#about_us div.widgetContainer{
		width: 100%;
	}




							
								








				


			
			







		
			 
 
Call us today 
for a free consultation:
(111) 222-3333 

		







 
			    			













				


			
			





Unique Test Data













				


			
			








A Test Firm







	/* The #nav is for sw4 sites only. go back and check them and change this back or something? */
	/* #sidebar-nav ul is for layout1 only */
	nav ul li,#nav ul li {
		float: left;
		list-style-type: none;
		margin: 0;
		padding: 0;
	}

	nav ul li a,
	nav ul li a:active,
	#nav ul li a,
	#nav ul li a:active {
		display: block;
		text-decoration: none;
	}

	nav ul li a:hover,
	#nav ul li a:hover {
		text-decoration: none;
	}

	nav ul li ul li,
	#nav ul li ul li {
		float: none;	
	}
	
	nav ul ul,
	#nav ul ul,
	#sidebar-nav ul {
		display:none;
		list-style:none;
	}
	nav select#select-nav,
	#nav select#select-nav,
	#select-nav {
		display:none;
	}
	/* Temp oldschool-horiz fix */
	ul.tabs ul ul {
		padding-bottom: 2px;
	}
	ul.tabs ul {
		padding-bottom: 8px;
	}

		
		HomeAbout Our FirmServicesLinksClient LoginContactTax ToolsTax TipsIndividualBusinessFinancialTax RatesDue DatesFinancial ToolsRetention GuideIRS FormsNewsMonthly NewsDaily NewsFinancial GuidesBankingBusinessFinancialInsuranceLife EventsTaxes
















				


			
			











				


			
			







	.widgetContainer {		
		position: relative;
		margin-left: 0;
		margin-bottom: 0;
		margin-right: 0;
		margin-top: 0;
		padding-top: 0;
		padding-right: 0;
		padding-bottom: 0;
		padding-left: 0;
	}
	#about_us div.widgetContainer{
		width: 100%;
	}




							
								











				


			
			



			
				.s3sliderImage span { visibility: hidden; }
			
			
			
			
			
			
				
				
						
							
							
						
						
							
							
						
						
							
							
						
						
							
							
						
					
				
			
		
		
			jQuery(document).ready(function(){
				jQuery(&quot; , &quot;'&quot; , &quot;#s3slider&quot; , &quot;'&quot; , &quot;).s3Slider({
					timeOut: 4000
				});
			});
		
	
	
	
	
  	
  	
  	
 
			    			













				


			
			











				


			
			







	.widgetContainer {		
		position: relative;
		margin-left: 0;
		margin-bottom: 0;
		margin-right: 0;
		margin-top: 0;
		padding-top: 0;
		padding-right: 0;
		padding-bottom: 0;
		padding-left: 0;
	}
	#about_us div.widgetContainer{
		width: 100%;
	}




							
								








				


			
			




	#login_widget_submit { display: block; }

#onvioSignIn #login_widget_submit, #loginForm #login_widget_submit{
margin: 15px auto;
padding: 20px 25px 20px 25px;
border-radius: 5px;
width: 125px;
height: 50px;
font-size: 16px!important;
line-height: 10px!important;
float: none;
text-align: center!important;

}



    function resetLoginForm() { // submits form
        document.getElementById(&quot;loginForm&quot;).reset();
    }
    function btnLoginClickAction()
    {
        if (document.getElementById(&quot;loginForm&quot;)) {
            setTimeout(&quot;resetLoginForm()&quot;, 5000); // set timout 
       }
    }

			
			
				Client Login
				
					
						
						
					
						Login
					
					
			    
			



 
			    			
							
								








				


			
			




	
	
	
	
	
	

			
	            
	                    Subscribe to our Newsletter
									
	                    	                    
										
	                    
	                    
	                    Submit
	                    
	            
           
		
		

jQuery(document).ready(function(){
	jQuery.validate({
		validateOnBlur: true,
		form : &quot; , &quot;'&quot; , &quot;#newslettersignupwidget&quot; , &quot;'&quot; , &quot;,
		onSuccess: function(){
			newsletterSignUp();			
			return false;
		}
	});	
	
});
	
function newsletterSignUp(){
	var email = jQuery(&quot;#965339 .newsletterSignupEmail&quot;).val();
	  jQuery(&quot;#965339 .emailCheckIt&quot;).val(email);
	  var name = jQuery(&quot;.newsletterSignupName&quot;).val();
  	  var siteId = &quot;14905&quot;;
  	  jQuery.post(&quot;utilities/add-to-mailinglist.jsp&quot;, { name: name, email: email, site_id:siteId }, function(data){
	  	var result = data.status;
		if(result != &quot;error&quot;){
			var message = &quot;&lt;h5 style=&quot; , &quot;'&quot; , &quot;text-align:center;&quot; , &quot;'&quot; , &quot;>&quot; + &quot;Success!&quot; + &quot;&lt;/h5>&lt;p style=&quot; , &quot;'&quot; , &quot;font-weight:normal;text-align:center;height:auto;float:none;&quot; , &quot;'&quot; , &quot;>&quot; + &quot;You have been signed up for the mailing list.&quot; + &quot;&lt;/p>&quot;
			jQuery(&quot;#965339 .newsletterSignupDiv&quot;).html(message);
		}
	 });
}
	
	

 
			    			














				


			
			











				


			
			







	.widgetContainer {		
		position: relative;
		margin-left: 0;
		margin-bottom: 0;
		margin-right: 0;
		margin-top: 0;
		padding-top: 0;
		padding-right: 0;
		padding-bottom: 0;
		padding-left: 0;
	}
	#about_us div.widgetContainer{
		width: 100%;
	}

















				


			
			







	             	Home
	             	
	             	About Our Firm
	             	
	             	Services
	             	
	             	Client Login
	             	
	             	Contact
	             	
	             	Glossary
	             	
	             	Links
	             	












				


			
			














				


			
			




	
		.social_link_anchor { width: 33px; height: 32px;display:inline-block; }
		.twitter_anchor { background: url(emoAssets/images/twitter.png) no-repeat; }
		.facebook_anchor { background: url(emoAssets/images/facebook.png) no-repeat; }
		.linkedin_anchor { background: url(emoAssets/images/linkedin.png) no-repeat; }
		.yelp_anchor { background: url(emoAssets/images/yelp.png) no-repeat; }
		#get_footer_social ul li { list-style: none;display: inline; padding: 5px; }
		#get_footer_social { display: block; text-align:center; width: 100%; margin: 5px auto; }
		.social_links_widget li { list-style:none; }
	

			
				
				
				
			




© 2023 Unique Test Data   All Rights Reserved.
 	Web Builder CS: Websites for Accountants






&quot;) or . = concat(&quot;













				


			
			











				


			
			







	.widgetContainer {		
		position: relative;
		margin-left: 0;
		margin-bottom: 0;
		margin-right: 0;
		margin-top: 0;
		padding-top: 0;
		padding-right: 0;
		padding-bottom: 0;
		padding-left: 0;
	}
	#about_us div.widgetContainer{
		width: 100%;
	}




							
								








				


			
			







		
			 
 
Call us today 
for a free consultation:
(111) 222-3333 

		







 
			    			













				


			
			





Unique Test Data













				


			
			








A Test Firm







	/* The #nav is for sw4 sites only. go back and check them and change this back or something? */
	/* #sidebar-nav ul is for layout1 only */
	nav ul li,#nav ul li {
		float: left;
		list-style-type: none;
		margin: 0;
		padding: 0;
	}

	nav ul li a,
	nav ul li a:active,
	#nav ul li a,
	#nav ul li a:active {
		display: block;
		text-decoration: none;
	}

	nav ul li a:hover,
	#nav ul li a:hover {
		text-decoration: none;
	}

	nav ul li ul li,
	#nav ul li ul li {
		float: none;	
	}
	
	nav ul ul,
	#nav ul ul,
	#sidebar-nav ul {
		display:none;
		list-style:none;
	}
	nav select#select-nav,
	#nav select#select-nav,
	#select-nav {
		display:none;
	}
	/* Temp oldschool-horiz fix */
	ul.tabs ul ul {
		padding-bottom: 2px;
	}
	ul.tabs ul {
		padding-bottom: 8px;
	}

		
		HomeAbout Our FirmServicesLinksClient LoginContactTax ToolsTax TipsIndividualBusinessFinancialTax RatesDue DatesFinancial ToolsRetention GuideIRS FormsNewsMonthly NewsDaily NewsFinancial GuidesBankingBusinessFinancialInsuranceLife EventsTaxes
















				


			
			











				


			
			







	.widgetContainer {		
		position: relative;
		margin-left: 0;
		margin-bottom: 0;
		margin-right: 0;
		margin-top: 0;
		padding-top: 0;
		padding-right: 0;
		padding-bottom: 0;
		padding-left: 0;
	}
	#about_us div.widgetContainer{
		width: 100%;
	}




							
								











				


			
			



			
				.s3sliderImage span { visibility: hidden; }
			
			
			
			
			
			
				
				
						
							
							
						
						
							
							
						
						
							
							
						
						
							
							
						
					
				
			
		
		
			jQuery(document).ready(function(){
				jQuery(&quot; , &quot;'&quot; , &quot;#s3slider&quot; , &quot;'&quot; , &quot;).s3Slider({
					timeOut: 4000
				});
			});
		
	
	
	
	
  	
  	
  	
 
			    			













				


			
			











				


			
			







	.widgetContainer {		
		position: relative;
		margin-left: 0;
		margin-bottom: 0;
		margin-right: 0;
		margin-top: 0;
		padding-top: 0;
		padding-right: 0;
		padding-bottom: 0;
		padding-left: 0;
	}
	#about_us div.widgetContainer{
		width: 100%;
	}




							
								








				


			
			




	#login_widget_submit { display: block; }

#onvioSignIn #login_widget_submit, #loginForm #login_widget_submit{
margin: 15px auto;
padding: 20px 25px 20px 25px;
border-radius: 5px;
width: 125px;
height: 50px;
font-size: 16px!important;
line-height: 10px!important;
float: none;
text-align: center!important;

}



    function resetLoginForm() { // submits form
        document.getElementById(&quot;loginForm&quot;).reset();
    }
    function btnLoginClickAction()
    {
        if (document.getElementById(&quot;loginForm&quot;)) {
            setTimeout(&quot;resetLoginForm()&quot;, 5000); // set timout 
       }
    }

			
			
				Client Login
				
					
						
						
					
						Login
					
					
			    
			



 
			    			
							
								








				


			
			




	
	
	
	
	
	

			
	            
	                    Subscribe to our Newsletter
									
	                    	                    
										
	                    
	                    
	                    Submit
	                    
	            
           
		
		

jQuery(document).ready(function(){
	jQuery.validate({
		validateOnBlur: true,
		form : &quot; , &quot;'&quot; , &quot;#newslettersignupwidget&quot; , &quot;'&quot; , &quot;,
		onSuccess: function(){
			newsletterSignUp();			
			return false;
		}
	});	
	
});
	
function newsletterSignUp(){
	var email = jQuery(&quot;#965339 .newsletterSignupEmail&quot;).val();
	  jQuery(&quot;#965339 .emailCheckIt&quot;).val(email);
	  var name = jQuery(&quot;.newsletterSignupName&quot;).val();
  	  var siteId = &quot;14905&quot;;
  	  jQuery.post(&quot;utilities/add-to-mailinglist.jsp&quot;, { name: name, email: email, site_id:siteId }, function(data){
	  	var result = data.status;
		if(result != &quot;error&quot;){
			var message = &quot;&lt;h5 style=&quot; , &quot;'&quot; , &quot;text-align:center;&quot; , &quot;'&quot; , &quot;>&quot; + &quot;Success!&quot; + &quot;&lt;/h5>&lt;p style=&quot; , &quot;'&quot; , &quot;font-weight:normal;text-align:center;height:auto;float:none;&quot; , &quot;'&quot; , &quot;>&quot; + &quot;You have been signed up for the mailing list.&quot; + &quot;&lt;/p>&quot;
			jQuery(&quot;#965339 .newsletterSignupDiv&quot;).html(message);
		}
	 });
}
	
	

 
			    			














				


			
			











				


			
			







	.widgetContainer {		
		position: relative;
		margin-left: 0;
		margin-bottom: 0;
		margin-right: 0;
		margin-top: 0;
		padding-top: 0;
		padding-right: 0;
		padding-bottom: 0;
		padding-left: 0;
	}
	#about_us div.widgetContainer{
		width: 100%;
	}

















				


			
			







	             	Home
	             	
	             	About Our Firm
	             	
	             	Services
	             	
	             	Client Login
	             	
	             	Contact
	             	
	             	Glossary
	             	
	             	Links
	             	












				


			
			














				


			
			




	
		.social_link_anchor { width: 33px; height: 32px;display:inline-block; }
		.twitter_anchor { background: url(emoAssets/images/twitter.png) no-repeat; }
		.facebook_anchor { background: url(emoAssets/images/facebook.png) no-repeat; }
		.linkedin_anchor { background: url(emoAssets/images/linkedin.png) no-repeat; }
		.yelp_anchor { background: url(emoAssets/images/yelp.png) no-repeat; }
		#get_footer_social ul li { list-style: none;display: inline; padding: 5px; }
		#get_footer_social { display: block; text-align:center; width: 100%; margin: 5px auto; }
		.social_links_widget li { list-style:none; }
	

			
				
				
				
			




© 2023 Unique Test Data   All Rights Reserved.
 	Web Builder CS: Websites for Accountants






&quot;))]</value>
      <webElementGuid>f9a74f17-e65e-4a93-902b-9d4ef6240447</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
