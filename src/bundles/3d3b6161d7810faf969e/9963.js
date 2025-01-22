"use strict";(self.webpackChunkelement_web=self.webpackChunkelement_web||[]).push([[395,9963],{"./res/img/external-link.svg":(e,t,o)=>{o.d(t,{I:()=>i});var n,a=o("./node_modules/react/index.js");function s(){return s=Object.assign?Object.assign.bind():function(e){for(var t=1;t<arguments.length;t++){var o=arguments[t];for(var n in o)({}).hasOwnProperty.call(o,n)&&(e[n]=o[n])}return e},s.apply(null,arguments)}var r=function(e,t){return a.createElement("svg",s({xmlns:"http://www.w3.org/2000/svg",viewBox:"0 0 11 10",role:"presentation","aria-hidden":!0,ref:t},e),n||(n=a.createElement("path",{fill:"none",fillRule:"evenodd",stroke:"currentColor",strokeLinecap:"round",strokeLinejoin:"round",d:"M8.5 5.5v3a1 1 0 0 1-1 1H2a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1h3M7 .5h3v3M4.5 6 10 .5"})))},i=(0,a.forwardRef)(r)},"./src/components/views/beacon/BeaconViewDialog.tsx":(e,t,o)=>{o.r(t),o.d(t,{default:()=>H});var n=o("./node_modules/react/index.js"),a=o("./res/img/location/live-location.svg"),s=o("./node_modules/matrix-js-sdk/src/matrix.ts"),r=o("./src/hooks/useEventEmitter.ts");var i=o("./src/contexts/MatrixClientContext.tsx"),l=o("./src/components/views/dialogs/BaseDialog.tsx"),c=o("./src/components/views/location/Map.tsx"),m=o("./src/components/views/location/ZoomButtons.tsx"),d=o("./src/components/views/location/index.tsx");const u=({map:e,beacon:t,tooltip:o})=>{var a;const l=(0,r.dF)(t,s.BeaconEvent.LocationUpdate,(()=>t.latestLocationState)),c=(0,n.useContext)(i.Ay).getRoom(t.roomId);if(!l||!t.isLive)return null;const m=l.uri||"",u=(null===(a=t.beaconInfo)||void 0===a?void 0:a.assetType)===s.LocationAssetType.Self,v=null==c?void 0:c.getMember(t.beaconInfoOwner),_=u&&v?v:void 0;return n.createElement(d.U3,{map:e,id:t.identifier,geoUri:m,roomMember:_,tooltip:o,useMemberColor:!0})};var v=o("./src/utils/arrays.ts"),_=o("./src/utils/location/index.ts");const p=e=>{const t=(0,v.Bo)(e.map((e=>{var t;return null!==(t=e.latestLocationState)&&void 0!==t&&t.uri?(0,_.XB)(e.latestLocationState.uri):void 0})));if(!t.length)return;const o=[...t].sort(((e,t)=>t.latitude-e.latitude)),n=[...t].sort(((e,t)=>t.longitude-e.longitude));return o.length<1||n.length<1?void 0:{north:o[0].latitude,south:o[o.length-1].latitude,east:n[0].longitude,west:n[n.length-1].longitude}};var g=o("./src/utils/beacon/index.ts"),b=o("./src/languageHandler.tsx"),x=o("./src/components/views/elements/AccessibleButton.tsx"),f=o("./node_modules/@vector-im/compound-design-tokens/assets/web/icons/close.js"),w=o("./src/components/views/typography/Heading.tsx"),h=o("./node_modules/@babel/runtime/helpers/esm/extends.js"),E=o("./node_modules/@babel/runtime/helpers/esm/objectWithoutProperties.js"),A=o("./src/utils/humanize.ts"),y=o("./src/utils/NativeEventUtils.ts"),B=o("./src/components/views/avatars/MemberAvatar.tsx"),C=o("./src/components/views/beacon/BeaconStatus.tsx"),S=o("./src/components/views/beacon/displayStatus.ts"),L=o("./src/components/views/beacon/StyledLiveBeaconIcon.tsx"),I=o("./node_modules/@vector-im/compound-web/dist/components/Tooltip/Tooltip.js"),k=o("./res/img/external-link.svg"),N=o("./src/components/views/elements/CopyableText.tsx");const j=({latestLocationState:e})=>{const[t,o]=(0,n.useState)();if((0,n.useEffect)((()=>{if(null==e||!e.uri)return;const t=(0,_.XB)(e.uri);o(t)}),[e]),!e||!t)return null;const a=`${t.latitude},${t.longitude}`,s=(0,_.eC)(t);return n.createElement(n.Fragment,null,n.createElement(I.m,{label:(0,b._t)("timeline|context_menu|open_in_osm")},n.createElement("a",{href:s,target:"_blank",rel:"noreferrer noopener"},n.createElement(k.I,{className:"mx_ShareLatestLocation_icon"}))),n.createElement(N.A,{className:"mx_ShareLatestLocation_copy",border:!1,getTextToCopy:()=>a}))},M=["beacon"],D=e=>{var t,o;let{beacon:a}=e,l=(0,E.A)(e,M);const c=(0,r.dF)(a,s.BeaconEvent.LocationUpdate,(()=>a.latestLocationState)),m=(0,n.useContext)(i.Ay).getRoom(a.roomId);if(!c||!a.isLive||!m)return null;const d=(null===(t=a.beaconInfo)||void 0===t?void 0:t.assetType)===s.LocationAssetType.Self,u=d?m.getMember(a.beaconInfoOwner):null,v=c.timestamp&&(0,A.P)(c.timestamp)||"";return n.createElement("li",(0,h.A)({className:"mx_BeaconListItem"},l),d?n.createElement(B.A,{className:"mx_BeaconListItem_avatar",member:u,size:"32px"}):n.createElement(L.A,{className:"mx_BeaconListItem_avatarIcon"}),n.createElement("div",{className:"mx_BeaconListItem_info"},n.createElement(C.A,{className:"mx_BeaconListItem_status",beacon:a,label:(null==u?void 0:u.name)||(null===(o=a.beaconInfo)||void 0===o?void 0:o.description)||a.beaconInfoOwner,displayStatus:S.T.Active},n.createElement("div",{className:"mx_BeaconListItem_interactions",onClick:(0,y.Z)((()=>{}))},n.createElement(j,{latestLocationState:c}))),n.createElement("span",{className:"mx_BeaconListItem_lastUpdated"},(0,b._t)("location_sharing|live_update_time",{humanizedUpdateTime:v}))))},T=({beacons:e,onBeaconClick:t,requestClose:o})=>n.createElement("div",{className:"mx_DialogSidebar"},n.createElement("div",{className:"mx_DialogSidebar_header"},n.createElement(w.A,{size:"4"},(0,b._t)("action|view_list")),n.createElement(x.A,{className:"mx_DialogSidebar_closeButton",onClick:o,title:(0,b._t)("location_sharing|close_sidebar")},n.createElement(f.A,{className:"mx_DialogSidebar_closeButtonIcon",height:"24px",width:"24px"}))),null!=e&&e.length?n.createElement("ol",{className:"mx_DialogSidebar_list"},e.map((e=>n.createElement(D,{key:e.identifier,beacon:e,onClick:()=>t(e)})))):n.createElement("div",{className:"mx_DialogSidebar_noResults"},(0,b._t)("location_sharing|live_locations_empty")));var F=o("./src/stores/OwnBeaconStore.ts"),U=o("./src/components/views/beacon/OwnBeaconStatus.tsx");const O=({roomId:e})=>{var t;const o=(e=>(0,r.dF)(F.g.instance,F.q.LivenessChange,(()=>{const[t]=F.g.instance.getLiveBeaconIds(e);return F.g.instance.getBeaconById(t)})))(e),a=(0,n.useContext)(i.Ay).getRoom(e);if(null==o||!o.isLive||!a)return null;const l=(null===(t=o.beaconInfo)||void 0===t?void 0:t.assetType)===s.LocationAssetType.Self,c=l?a.getMember(o.beaconInfoOwner):null;return n.createElement("div",{className:"mx_DialogOwnBeaconStatus"},l?n.createElement(B.A,{className:"mx_DialogOwnBeaconStatus_avatar",member:c,size:"32px"}):n.createElement(L.A,{className:"mx_DialogOwnBeaconStatus_avatarIcon"}),n.createElement(U.A,{className:"mx_DialogOwnBeaconStatus_status",beacon:o,displayStatus:S.T.Active}))},R=({beacon:e})=>{const t=(e=>{var t;const o=(0,n.useContext)(i.Ay);var a;if((null===(t=e.beaconInfo)||void 0===t?void 0:t.assetType)!==s.LocationAssetType.Self)return null===(a=e.beaconInfo)||void 0===a?void 0:a.description;const r=o.getRoom(e.roomId),l=null==r?void 0:r.getMember(e.beaconInfoOwner);return(null==l?void 0:l.rawDisplayName)||e.beaconInfoOwner})(e);return n.createElement("div",{className:"mx_BeaconStatusTooltip"},n.createElement(C.A,{beacon:e,label:t,displayStatus:S.T.Active,displayLiveTimeRemaining:!0,className:"mx_BeaconStatusTooltip_inner"},n.createElement(j,{latestLocationState:e.latestLocationState})))};var G=o("./src/components/views/location/MapFallback.tsx"),z=o("./src/components/views/location/MapError.tsx");const Z=e=>{if(e)return(0,g.mt)({latitude:(e.north+e.south)/2,longitude:(e.east+e.west)/2,timestamp:Date.now()})},H=({initialFocusedBeacon:e,roomId:t,matrixClient:o,onFinished:d})=>{const v=((e,t)=>{const o=t.getRoom(e);return(0,r.dF)(null==o?void 0:o.currentState,s.RoomStateEvent.BeaconLiveness,(()=>{var e;return(null==o||null===(e=o.currentState)||void 0===e?void 0:e.liveBeaconIds.map((e=>o.currentState.beacons.get(e))))||[]}))})(t,o),[_,g]=(0,n.useState)({beacon:e,ts:0}),[f,w]=(0,n.useState)(!1),{bounds:h,centerGeoUri:E}=((e,{beacon:t,ts:o})=>{var a;const[s,r]=(0,n.useState)(p(e)),[i,l]=(0,n.useState)((null==t||null===(a=t.latestLocationState)||void 0===a?void 0:a.uri)||Z(s));return(0,n.useEffect)((()=>{var e,n;0!==o&&null!=t&&null!==(e=t.latestLocationState)&&void 0!==e&&e.uri&&(l(`${null==t||null===(n=t.latestLocationState)||void 0===n?void 0:n.uri};mxTs=${Date.now()}`),r(p([t])))}),[t,o]),{bounds:s,centerGeoUri:i}})(v,_),[A,y]=(0,n.useState)();(0,n.useEffect)((()=>{A&&w(!0)}),[A]);const B=v.filter((e=>(null==e?void 0:e.beaconInfoOwner)===o.getUserId())).length>0;return n.createElement(l.A,{className:"mx_BeaconViewDialog",onFinished:d,fixedWidth:!1},n.createElement(i.Ay.Provider,{value:o},E&&!A&&n.createElement(c.default,{id:"mx_BeaconViewDialog",bounds:h,centerGeoUri:E,interactive:!0,onError:y,className:"mx_BeaconViewDialog_map",allowGeolocate:!B},(({map:e})=>n.createElement(n.Fragment,null,v.map((t=>n.createElement(u,{key:t.identifier,map:e,beacon:t,tooltip:n.createElement(R,{beacon:t})}))),n.createElement(m.A,{map:e})))),A instanceof Error&&n.createElement(z.p,{error:A.message,isMinimised:!0}),!E&&!A&&n.createElement(G.A,{className:"mx_BeaconViewDialog_map"},n.createElement("span",{className:"mx_BeaconViewDialog_mapFallbackMessage"},(0,b._t)("location_sharing|live_locations_empty")),n.createElement(x.A,{kind:"primary",onClick:d},(0,b._t)("action|close"))),f?n.createElement(T,{beacons:v,onBeaconClick:e=>{g({beacon:e,ts:Date.now()})},requestClose:()=>w(!1)}):n.createElement(x.A,{kind:"primary",onClick:()=>w(!0),className:"mx_BeaconViewDialog_viewListButton"},n.createElement(a.I,{height:12})," ",(0,b._t)("action|view_list")),n.createElement(O,{roomId:t})))}},"./src/components/views/location/Map.tsx":(e,t,o)=>{o.r(t),o.d(t,{default:()=>f});var n=o("./node_modules/react/index.js"),a=o("./node_modules/classnames/index.js"),s=o.n(a),r=o("./node_modules/maplibre-gl/dist/maplibre-gl.js"),i=o("./node_modules/matrix-js-sdk/src/matrix.ts"),l=o("./node_modules/matrix-js-sdk/src/logger.ts"),c=o("./src/contexts/MatrixClientContext.tsx"),m=o("./src/hooks/useEventEmitter.ts"),d=o("./src/utils/location/index.ts"),u=o("./src/utils/WellKnownUtils.ts"),v=o("./src/utils/location/map.ts");var _=o("./src/Modal.tsx"),p=o("./src/components/views/dialogs/ErrorDialog.tsx"),g=o("./src/languageHandler.tsx");const b=({id:e,centerGeoUri:t,onError:o,interactive:a,bounds:s,allowGeolocate:_})=>{const p=`mx_Map_${e}`,g=(0,n.useContext)(c.Ay),b=(0,m.dF)(g,i.ClientEvent.ClientWellKnown,(e=>{var t;return null===(t=(0,u.XP)(e))||void 0===t?void 0:t.map_style_url})),f=(({interactive:e,bodyId:t,onError:o})=>{const a=(0,c.nH)(),[s,r]=(0,n.useState)();return(0,n.useEffect)((()=>{try{r((0,v.p)(a,!!e,t,o))}catch(e){console.error("Error encountered in useMap",e),e instanceof Error&&(null==o||o(e))}return()=>{s&&(s.remove(),r(void 0))}}),[e,t,o]),s})({interactive:a,bodyId:p,onError:o});(0,n.useEffect)((()=>{b&&f&&f.setStyle(b)}),[b,f]),(0,n.useEffect)((()=>{if(f&&t)try{const e=(0,d.XB)(t);if(!e)throw new Error("Invalid geo URI");f.setCenter({lon:e.longitude,lat:e.latitude})}catch(e){l.v.error("Could not set map center",e)}}),[f,t]),(0,n.useEffect)((()=>{if(f&&s)try{const e=new r.LngLatBounds([s.west,s.south],[s.east,s.north]);f.fitBounds(e,{padding:100,maxZoom:15})}catch(e){l.v.error("Invalid map bounds",e)}}),[f,s]);const[w,h]=(0,n.useState)(null);return(0,n.useEffect)((()=>{if(f){if(_&&!w){const e=new r.GeolocateControl({positionOptions:{enableHighAccuracy:!0},trackUserLocation:!1});h(e),f.addControl(e)}!_&&w&&(f.removeControl(w),h(null))}}),[f,w,_]),(0,n.useEffect)((()=>{if(w)return w.on("error",x),()=>{w.off("error",x)}}),[w]),{map:f,bodyId:p}},x=e=>{var t;l.v.error("Could not fetch location",e),_.Ay.createDialog(p.A,{title:(0,g._t)("location_sharing|error_fetch_location"),description:null!==(t=(0,d.Ff)(e.code))&&void 0!==t?t:""})},f=({bounds:e,centerGeoUri:t,children:o,className:a,allowGeolocate:r,id:i,interactive:l,onError:c,onClick:m})=>{const{map:d,bodyId:u}=b({centerGeoUri:t,onError:c,id:i,interactive:l,bounds:e,allowGeolocate:r});return n.createElement("div",{className:s()("mx_Map",a),id:u,onClick:e=>{e.target.classList.contains("maplibregl-ctrl-attrib-button")||null==m||m()}},!!o&&!!d&&o({map:d}))}},"./src/components/views/location/ZoomButtons.tsx":(e,t,o)=>{o.d(t,{A:()=>l});var n=o("./node_modules/react/index.js"),a=o("./node_modules/@vector-im/compound-design-tokens/assets/web/icons/plus.js"),s=o("./node_modules/@vector-im/compound-design-tokens/assets/web/icons/minus.js"),r=o("./src/languageHandler.tsx"),i=o("./src/components/views/elements/AccessibleButton.tsx");const l=({map:e})=>n.createElement("div",{className:"mx_ZoomButtons"},n.createElement(i.A,{onClick:()=>{e.zoomIn()},title:(0,r._t)("action|zoom_in"),className:"mx_ZoomButtons_button"},n.createElement(a.A,{className:"mx_ZoomButtons_icon"})),n.createElement(i.A,{onClick:()=>{e.zoomOut()},title:(0,r._t)("action|zoom_out"),className:"mx_ZoomButtons_button"},n.createElement(s.A,{className:"mx_ZoomButtons_icon"})))},"./src/utils/location/map.ts":(e,t,o)=>{o.d(t,{h:()=>c,p:()=>l});var n=o("./node_modules/maplibre-gl/dist/maplibre-gl.js"),a=o("./node_modules/matrix-js-sdk/src/logger.ts"),s=o("./src/languageHandler.tsx"),r=o("./src/utils/location/findMapStyleUrl.ts"),i=o("./src/utils/location/LocationShareErrors.ts");const l=(e,t,o,l)=>{try{const c=(0,r.M)(e),m=new n.Map({container:o,style:c,zoom:15,interactive:t,attributionControl:!1,locale:{"AttributionControl.ToggleAttribution":(0,s._t)("location_sharing|toggle_attribution"),"AttributionControl.MapFeedback":(0,s._t)("location_sharing|map_feedback"),"FullscreenControl.Enter":(0,s._t)("action|enter_fullscreen"),"FullscreenControl.Exit":(0,s._t)("action|exit_fullscreeen"),"GeolocateControl.FindMyLocation":(0,s._t)("location_sharing|find_my_location"),"GeolocateControl.LocationNotAvailable":(0,s._t)("location_sharing|location_not_available"),"LogoControl.Title":(0,s._t)("location_sharing|mapbox_logo"),"NavigationControl.ResetBearing":(0,s._t)("location_sharing|reset_bearing"),"NavigationControl.ZoomIn":(0,s._t)("action|zoom_in"),"NavigationControl.ZoomOut":(0,s._t)("action|zoom_out")}});return m.addControl(new n.AttributionControl,"top-right"),m.on("error",(e=>{a.v.error("Failed to load map: check map_style_url in config.json has a valid URL and API key",e.error),null==l||l(new Error(i.$.MapStyleUrlNotReachable))})),m}catch(e){a.v.error("Failed to render map",e);if((null==e?void 0:e.message).includes("Failed to initialize WebGL"))throw new Error(i.$.WebGLNotEnabled);throw e}},c=(e,t)=>new n.Marker({element:t,anchor:"bottom",offset:[0,-1]}).setLngLat({lon:e.longitude,lat:e.latitude})},"./node_modules/@vector-im/compound-design-tokens/assets/web/icons/minus.js":(e,t,o)=>{o.d(t,{A:()=>r});var n=o("./node_modules/react/index.js"),a=o("./node_modules/react/jsx-runtime.js");function s(e,t){return(0,a.jsx)("svg",{xmlns:"http://www.w3.org/2000/svg",width:"1em",height:"1em",fill:"currentColor",viewBox:"0 0 24 24",ref:t,...e,children:(0,a.jsx)("path",{d:"M6 13a.967.967 0 0 1-.713-.287A.968.968 0 0 1 5 12c0-.283.096-.52.287-.713A.967.967 0 0 1 6 11h12a.97.97 0 0 1 .712.287c.192.192.288.43.288.713s-.096.52-.288.713A.968.968 0 0 1 18 13H6Z"})})}s.displayName="MinusIcon";const r=(0,n.forwardRef)(s)},"./node_modules/@vector-im/compound-design-tokens/assets/web/icons/plus.js":(e,t,o)=>{o.d(t,{A:()=>r});var n=o("./node_modules/react/index.js"),a=o("./node_modules/react/jsx-runtime.js");function s(e,t){return(0,a.jsx)("svg",{xmlns:"http://www.w3.org/2000/svg",width:"1em",height:"1em",fill:"currentColor",viewBox:"0 0 24 24",ref:t,...e,children:(0,a.jsx)("path",{d:"M11 13H6a.967.967 0 0 1-.713-.287A.968.968 0 0 1 5 12c0-.283.096-.52.287-.713A.967.967 0 0 1 6 11h5V6c0-.283.096-.52.287-.713A.968.968 0 0 1 12 5c.283 0 .52.096.713.287.191.192.287.43.287.713v5h5a.97.97 0 0 1 .712.287c.192.192.288.43.288.713s-.096.52-.288.713A.968.968 0 0 1 18 13h-5v5a.97.97 0 0 1-.287.712A.968.968 0 0 1 12 19a.968.968 0 0 1-.713-.288A.968.968 0 0 1 11 18v-5Z"})})}s.displayName="PlusIcon";const r=(0,n.forwardRef)(s)}}]);
//# sourceMappingURL=9963.js.map