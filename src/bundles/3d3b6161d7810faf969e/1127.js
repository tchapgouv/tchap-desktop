(self.webpackChunkelement_web=self.webpackChunkelement_web||[]).push([[1127],{"./src/async-components/views/dialogs/security/CreateSecretStorageDialog.tsx":(e,t,s)=>{"use strict";s.r(t),s.d(t,{default:()=>D});var a=s("./node_modules/@babel/runtime/helpers/esm/defineProperty.js"),r=s("./node_modules/react/index.js"),n=s("./node_modules/file-saver/dist/FileSaver.min.js"),o=s.n(n),i=s("./node_modules/matrix-js-sdk/src/logger.ts"),c=s("./node_modules/matrix-js-sdk/src/matrix.ts"),l=s("./node_modules/classnames/index.js"),p=s.n(l),u=s("./node_modules/@vector-im/compound-design-tokens/assets/web/icons/check.js"),h=s("./src/MatrixClientPeg.ts"),d=s("./src/languageHandler.tsx"),m=s("./src/Modal.tsx"),_=s("./src/utils/strings.ts"),y=s("./src/components/views/auth/InteractiveAuthEntryComponents.tsx"),g=s("./src/components/views/auth/PassphraseField.tsx"),k=s("./src/components/views/elements/StyledRadioButton.tsx"),b=s("./src/components/views/elements/AccessibleButton.tsx"),v=s("./src/components/views/elements/DialogButtons.tsx"),S=s("./src/components/views/elements/InlineSpinner.tsx"),f=s("./src/utils/WellKnownUtils.ts"),P=s("./src/modules/ModuleRunner.ts"),C=s("./src/components/views/dialogs/BaseDialog.tsx"),w=s("./src/components/views/elements/Spinner.tsx"),x=s("./src/components/views/dialogs/InteractiveAuthDialog.tsx"),A=s("./src/components/views/auth/PassphraseConfirmField.tsx"),E=s("./src/utils/device/dehydration.ts"),K=function(e){return e.Loading="loading",e.LoadError="load_error",e.ChooseKeyPassphrase="choose_key_passphrase",e.Passphrase="passphrase",e.PassphraseConfirm="passphrase_confirm",e.ShowKey="show_key",e.Storing="storing",e.Stored="stored",e.ConfirmSkip="confirm_skip",e}(K||{});class D extends r.PureComponent{constructor(e){super(e),(0,a.A)(this,"recoveryKey",void 0),(0,a.A)(this,"recoveryKeyNode",(0,r.createRef)()),(0,a.A)(this,"passphraseField",(0,r.createRef)()),(0,a.A)(this,"onKeyPassphraseChange",(e=>{this.setState({passPhraseKeySelected:e.target.value})})),(0,a.A)(this,"onChooseKeyPassphraseFormSubmit",(async()=>{this.state.passPhraseKeySelected===f.Yc.Key?(this.recoveryKey=await h.J.safeGet().getCrypto().createRecoveryKeyFromPassphrase(),this.setState({copied:!1,downloaded:!1,setPassphrase:!1,phase:K.ShowKey})):this.setState({copied:!1,downloaded:!1,phase:K.Passphrase})})),(0,a.A)(this,"onCopyClick",(()=>{(0,_.Ud)(this.recoveryKeyNode.current)&&this.setState({copied:!0})})),(0,a.A)(this,"onDownloadClick",(()=>{if(!this.recoveryKey)return;const e=new Blob([this.recoveryKey.encodedPrivateKey],{type:"text/plain;charset=us-ascii"});o().saveAs(e,"security-key.txt"),this.setState({downloaded:!0})})),(0,a.A)(this,"doBootstrapUIAuth",(async e=>{if(this.state.canUploadKeysWithPasswordOnly&&this.state.accountPassword)await e({type:"m.login.password",identifier:{type:"m.id.user",user:h.J.safeGet().getSafeUserId()},password:this.state.accountPassword});else{const t={[y.av.PHASE_PREAUTH]:{title:(0,d._t)("auth|uia|sso_title"),body:(0,d._t)("auth|uia|sso_preauth_body"),continueText:(0,d._t)("auth|sso"),continueKind:"primary"},[y.av.PHASE_POSTAUTH]:{title:(0,d._t)("encryption|confirm_encryption_setup_title"),body:(0,d._t)("encryption|confirm_encryption_setup_body"),continueText:(0,d._t)("action|confirm"),continueKind:"primary"}},{finished:s}=m.Ay.createDialog(x.A,{title:(0,d._t)("encryption|bootstrap_title"),matrixClient:h.J.safeGet(),makeRequest:e,aestheticsForStagePhases:{[y.av.LOGIN_TYPE]:t,[y.av.UNSTABLE_LOGIN_TYPE]:t}}),[a]=await s;if(!a)throw new Error("Cross-signing key upload auth canceled")}})),(0,a.A)(this,"bootstrapSecretStorage",(async()=>{const e=h.J.safeGet().getCrypto(),{forceReset:t,resetCrossSigning:s}=this.props;let a;if(!t)try{this.setState({phase:K.Loading}),a=await e.getKeyBackupInfo()}catch(e){return i.v.error("Error fetching backup data from server",e),void this.setState({phase:K.LoadError})}this.setState({phase:K.Storing,error:void 0});try{t?(i.v.log("Forcing secret storage reset"),await e.bootstrapSecretStorage({createSecretStorageKey:async()=>this.recoveryKey,setupNewSecretStorage:!0}),s&&(i.v.log("Resetting cross signing"),await e.bootstrapCrossSigning({authUploadDeviceSigningKeys:this.doBootstrapUIAuth,setupNewCrossSigning:!0})),i.v.log("Resetting key backup"),await e.resetKeyBackup()):(await e.bootstrapCrossSigning({authUploadDeviceSigningKeys:this.doBootstrapUIAuth}),await e.bootstrapSecretStorage({createSecretStorageKey:async()=>this.recoveryKey,setupNewKeyBackup:!a})),await(0,E.b)(!0),this.setState({phase:K.Stored})}catch(e){this.setState({error:!0}),i.v.error("Error bootstrapping secret storage",e)}})),(0,a.A)(this,"onCancel",(()=>{this.props.onFinished(!1)})),(0,a.A)(this,"onLoadRetryClick",(()=>{this.bootstrapSecretStorage()})),(0,a.A)(this,"onShowKeyContinueClick",(()=>{this.bootstrapSecretStorage()})),(0,a.A)(this,"onCancelClick",(()=>{this.setState({phase:K.ConfirmSkip})})),(0,a.A)(this,"onGoBackClick",(()=>{this.setState({phase:K.ChooseKeyPassphrase})})),(0,a.A)(this,"onPassPhraseNextClick",(async e=>{if(e.preventDefault(),this.passphraseField.current){if(await this.passphraseField.current.validate({allowEmpty:!1}),!this.passphraseField.current.state.valid)return this.passphraseField.current.focus(),void this.passphraseField.current.validate({allowEmpty:!1,focused:!0});this.setState({phase:K.PassphraseConfirm})}})),(0,a.A)(this,"onPassPhraseConfirmNextClick",(async e=>{e.preventDefault(),this.state.passPhrase===this.state.passPhraseConfirm&&(this.recoveryKey=await h.J.safeGet().getCrypto().createRecoveryKeyFromPassphrase(this.state.passPhrase),this.setState({copied:!1,downloaded:!1,setPassphrase:!0,phase:K.ShowKey}))})),(0,a.A)(this,"onSetAgainClick",(()=>{this.setState({passPhrase:"",passPhraseValid:!1,passPhraseConfirm:"",phase:K.Passphrase})})),(0,a.A)(this,"onPassPhraseValidate",(e=>{this.setState({passPhraseValid:!!e.valid})})),(0,a.A)(this,"onPassPhraseChange",(e=>{this.setState({passPhrase:e.target.value})})),(0,a.A)(this,"onPassPhraseConfirmChange",(e=>{this.setState({passPhraseConfirm:e.target.value})}));const t=h.J.safeGet();let s;s=(0,f.PR)(t).includes(f.Yc.Key)?f.Yc.Key:f.Yc.Passphrase;const n=e.accountPassword||"";let c=null;n&&(c=!0);const l=P.r.instance.extensions.cryptoSetup.createSecretStorageKey()?K.Loading:K.ChooseKeyPassphrase;this.state={phase:l,passPhrase:"",passPhraseValid:!1,passPhraseConfirm:"",copied:!1,downloaded:!1,setPassphrase:!1,accountPasswordCorrect:null,canSkip:!(0,f.R$)(t),canUploadKeysWithPasswordOnly:c,passPhraseKeySelected:s,accountPassword:n},this.showKeyPassphraseDirectly()}componentDidMount(){const e=P.r.instance.extensions.cryptoSetup.createSecretStorageKey();e&&this.initExtension(e),null===this.state.canUploadKeysWithPasswordOnly&&this.queryKeyUploadAuth()}async showKeyPassphraseDirectly(){try{this.recoveryKey=await h.J.safeGet().getCrypto().createRecoveryKeyFromPassphrase(),this.setState({copied:!1,downloaded:!1,setPassphrase:!1,phase:K.ShowKey,passPhraseKeySelected:f.Yc.Key})}catch(e){return console.error("Error fetching backup data from server",e),void this.setState({phase:K.LoadError})}}initExtension(e){i.v.log("CryptoSetupExtension: Created key via extension, jumping to bootstrap step"),this.recoveryKey={privateKey:e},this.bootstrapSecretStorage()}async queryKeyUploadAuth(){try{await h.J.safeGet().uploadDeviceSigningKeys(void 0,{}),i.v.log("uploadDeviceSigningKeys unexpectedly succeeded without UI auth!")}catch(e){if(!(e instanceof c.MatrixError&&e.data&&e.data.flows))return void i.v.log("uploadDeviceSigningKeys advertised no flows!");const t=e.data.flows.some((e=>1===e.stages.length&&"m.login.password"===e.stages[0]));this.setState({canUploadKeysWithPasswordOnly:t})}}renderOptionKey(){return r.createElement(k.A,{key:f.Yc.Key,value:f.Yc.Key,name:"keyPassphrase",checked:this.state.passPhraseKeySelected===f.Yc.Key,onChange:this.onKeyPassphraseChange,outlined:!0},r.createElement("div",{className:"mx_CreateSecretStorageDialog_optionTitle"},r.createElement("span",{className:"mx_CreateSecretStorageDialog_optionIcon mx_CreateSecretStorageDialog_optionIcon_secureBackup"}),(0,d._t)("settings|key_backup|setup_secure_backup|generate_security_key_title")),r.createElement("div",null,(0,d._t)("settings|key_backup|setup_secure_backup|generate_security_key_description")))}renderOptionPassphrase(){return r.createElement(k.A,{key:f.Yc.Passphrase,value:f.Yc.Passphrase,name:"keyPassphrase",checked:this.state.passPhraseKeySelected===f.Yc.Passphrase,onChange:this.onKeyPassphraseChange,outlined:!0},r.createElement("div",{className:"mx_CreateSecretStorageDialog_optionTitle"},r.createElement("span",{className:"mx_CreateSecretStorageDialog_optionIcon mx_CreateSecretStorageDialog_optionIcon_securePhrase"}),(0,d._t)("settings|key_backup|setup_secure_backup|enter_phrase_title")),r.createElement("div",null,(0,d._t)("settings|key_backup|setup_secure_backup|use_phrase_only_you_know")))}renderPhaseChooseKeyPassphrase(){const e=(0,f.PR)(h.J.safeGet()),t=e.includes(f.Yc.Key)?this.renderOptionKey():null,s=e.includes(f.Yc.Passphrase)?this.renderOptionPassphrase():null;return r.createElement("form",{onSubmit:this.onChooseKeyPassphraseFormSubmit},r.createElement("p",{className:"mx_CreateSecretStorageDialog_centeredBody"},(0,d._t)("settings|key_backup|setup_secure_backup|description")),r.createElement("div",{className:"mx_CreateSecretStorageDialog_primaryContainer",role:"radiogroup"},t,s),r.createElement(v.A,{primaryButton:(0,d._t)("action|continue"),onPrimaryButtonClick:this.onChooseKeyPassphraseFormSubmit,onCancel:this.onCancelClick,hasCancel:this.state.canSkip}))}renderPhasePassPhrase(){return r.createElement("form",{onSubmit:this.onPassPhraseNextClick},r.createElement("p",null,(0,d._t)("settings|key_backup|setup_secure_backup|enter_phrase_description")),r.createElement("div",{className:"mx_CreateSecretStorageDialog_passPhraseContainer"},r.createElement(g.A,{id:"mx_passPhraseInput",className:"mx_CreateSecretStorageDialog_passPhraseField",onChange:this.onPassPhraseChange,minScore:4,value:this.state.passPhrase,onValidate:this.onPassPhraseValidate,fieldRef:this.passphraseField,autoFocus:!0,label:(0,d.AO)("settings|key_backup|setup_secure_backup|enter_phrase_title"),labelEnterPassword:(0,d.AO)("settings|key_backup|setup_secure_backup|enter_phrase_title"),labelStrongPassword:(0,d.AO)("settings|key_backup|setup_secure_backup|phrase_strong_enough"),labelAllowedButUnsafe:(0,d.AO)("settings|key_backup|setup_secure_backup|phrase_strong_enough")})),r.createElement(v.A,{primaryButton:(0,d._t)("action|continue"),onPrimaryButtonClick:this.onPassPhraseNextClick,hasCancel:!1,disabled:!this.state.passPhraseValid},r.createElement("button",{type:"button",onClick:this.onCancelClick,className:"danger"},(0,d._t)("action|cancel"))))}renderPhasePassPhraseConfirm(){let e,t,s;return this.state.passPhraseConfirm===this.state.passPhrase?(e=(0,d._t)("settings|key_backup|setup_secure_backup|pass_phrase_match_success"),t=(0,d._t)("settings|key_backup|setup_secure_backup|use_different_passphrase")):this.state.passPhrase.startsWith(this.state.passPhraseConfirm)||(e=(0,d._t)("settings|key_backup|setup_secure_backup|pass_phrase_match_failed"),t=(0,d._t)("settings|key_backup|setup_secure_backup|set_phrase_again")),e&&(s=r.createElement("div",null,r.createElement("div",null,e),r.createElement(b.A,{kind:"link",onClick:this.onSetAgainClick},t))),r.createElement("form",{onSubmit:this.onPassPhraseConfirmNextClick},r.createElement("p",null,(0,d._t)("settings|key_backup|setup_secure_backup|enter_phrase_to_confirm")),r.createElement("div",{className:"mx_CreateSecretStorageDialog_passPhraseContainer"},r.createElement(A.A,{id:"mx_passPhraseInput",onChange:this.onPassPhraseConfirmChange,value:this.state.passPhraseConfirm,className:"mx_CreateSecretStorageDialog_passPhraseField",label:(0,d.AO)("settings|key_backup|setup_secure_backup|confirm_security_phrase"),labelRequired:(0,d.AO)("settings|key_backup|setup_secure_backup|confirm_security_phrase"),labelInvalid:(0,d.AO)("settings|key_backup|setup_secure_backup|pass_phrase_match_failed"),autoFocus:!0,password:this.state.passPhrase}),r.createElement("div",{className:"mx_CreateSecretStorageDialog_passPhraseMatch"},s)),r.createElement(v.A,{primaryButton:(0,d._t)("action|continue"),onPrimaryButtonClick:this.onPassPhraseConfirmNextClick,hasCancel:!1,disabled:this.state.passPhrase!==this.state.passPhraseConfirm},r.createElement("button",{type:"button",onClick:this.onCancelClick,className:"danger"},(0,d._t)("action|skip"))))}renderPhaseShowKey(){var e;let t;return t=this.state.phase===K.ShowKey?r.createElement(v.A,{primaryButton:(0,d._t)("I wrote down my code"),primaryDisabled:!this.state.downloaded&&!this.state.copied&&!this.state.setPassphrase,onPrimaryButtonClick:this.onShowKeyContinueClick,hasCancel:!0,onCancel:this.onCancel,cancelButtonClass:"mx_AccessibleButton mx_AccessibleButton_hasKind mx_AccessibleButton_kind_danger_outline"}):r.createElement("div",{className:"mx_CreateSecretStorageDialog_continueSpinner"},r.createElement(S.A,null)),r.createElement("div",null,r.createElement("p",null,(0,d._t)("This is your recovery key")),r.createElement("p",null,r.createElement("b",null,(0,d._t)("Warning: this is the only time this code will be displayed!"))),r.createElement("p",null,(0,d._t)("settings|key_backup|setup_secure_backup|security_key_safety_reminder")),r.createElement("div",{className:"mx_CreateSecretStorageDialog_primaryContainer mx_CreateSecretStorageDialog_recoveryKeyPrimarycontainer"},r.createElement("div",{className:"mx_CreateSecretStorageDialog_recoveryKeyContainer"},r.createElement("div",{className:"mx_CreateSecretStorageDialog_recoveryKey"},r.createElement("code",{ref:this.recoveryKeyNode},null===(e=this.recoveryKey)||void 0===e?void 0:e.encodedPrivateKey)),r.createElement("div",{className:"mx_CreateSecretStorageDialog_recoveryKeyButtons"},r.createElement(b.A,{kind:"primary",className:"mx_Dialog_primary mx_CreateSecretStorageDialog_recoveryKeyButtons_copyBtn",onClick:this.onCopyClick,disabled:this.state.phase===K.Storing},this.state.copied?(0,d._t)("common|copied"):(0,d._t)("action|copy"))))),t)}renderBusyPhase(){return r.createElement("div",null,r.createElement(w.A,null))}renderStoredPhase(){return r.createElement(r.Fragment,null,r.createElement("p",{className:"mx_Dialog_content"},(0,d._t)("settings|key_backup|setup_secure_backup|backup_setup_success_description")),r.createElement(v.A,{primaryButton:(0,d._t)("action|done"),onPrimaryButtonClick:()=>this.props.onFinished(!0),hasCancel:!1}))}renderPhaseLoadError(){return r.createElement("div",null,r.createElement("p",null,(0,d._t)("settings|key_backup|setup_secure_backup|secret_storage_query_failure")),r.createElement("div",{className:"mx_Dialog_buttons"},r.createElement(v.A,{primaryButton:(0,d._t)("action|retry"),onPrimaryButtonClick:this.onLoadRetryClick,hasCancel:this.state.canSkip,onCancel:this.onCancel})))}renderPhaseSkipConfirm(){return r.createElement("div",null,r.createElement("p",null,(0,d._t)("settings|key_backup|setup_secure_backup|cancel_warning")),r.createElement("p",null,(0,d._t)("settings|key_backup|setup_secure_backup|settings_reminder")),r.createElement(v.A,{primaryButton:(0,d._t)("action|go_back"),onPrimaryButtonClick:this.onGoBackClick,hasCancel:!1},r.createElement("button",{type:"button",className:"danger",onClick:this.onCancel},(0,d._t)("action|cancel"))))}titleForPhase(e){switch(e){case K.ChooseKeyPassphrase:return(0,d._t)("encryption|set_up_toast_title");case K.Passphrase:return(0,d._t)("settings|key_backup|setup_secure_backup|title_set_phrase");case K.PassphraseConfirm:return(0,d._t)("settings|key_backup|setup_secure_backup|title_confirm_phrase");case K.ConfirmSkip:return(0,d._t)("common|are_you_sure");case K.ShowKey:return(0,d._t)("settings|key_backup|setup_secure_backup|title_save_key");case K.Storing:return(0,d._t)("encryption|bootstrap_title");case K.Stored:return(0,d._t)("settings|key_backup|setup_secure_backup|backup_setup_success_title");default:return""}}get topComponent(){return this.state.phase===K.Stored?r.createElement(u.A,{className:"mx_Icon mx_Icon_circle-40 mx_Icon_accent mx_Icon_bg-accent-light"}):null}get classNames(){return p()("mx_CreateSecretStorageDialog",{mx_SuccessDialog:this.state.phase===K.Stored})}render(){let e,t;if(this.state.error)e=r.createElement("div",null,r.createElement("p",null,(0,d._t)("settings|key_backup|setup_secure_backup|unable_to_setup")),r.createElement("div",{className:"mx_Dialog_buttons"},r.createElement(v.A,{primaryButton:(0,d._t)("action|retry"),onPrimaryButtonClick:this.bootstrapSecretStorage,hasCancel:this.state.canSkip,onCancel:this.onCancel})));else switch(this.state.phase){case K.Loading:e=this.renderBusyPhase();break;case K.LoadError:e=this.renderPhaseLoadError();break;case K.ChooseKeyPassphrase:e=this.renderPhaseChooseKeyPassphrase();break;case K.Passphrase:e=this.renderPhasePassPhrase();break;case K.PassphraseConfirm:e=this.renderPhasePassPhraseConfirm();break;case K.ShowKey:e=this.renderPhaseShowKey();break;case K.Storing:e=this.renderBusyPhase();break;case K.Stored:e=this.renderStoredPhase();break;case K.ConfirmSkip:e=this.renderPhaseSkipConfirm()}switch(this.state.phase){case K.Passphrase:case K.PassphraseConfirm:t=["mx_CreateSecretStorageDialog_titleWithIcon","mx_CreateSecretStorageDialog_securePhraseTitle"];break;case K.ShowKey:t=["mx_CreateSecretStorageDialog_titleWithIcon","mx_CreateSecretStorageDialog_secureBackupTitle"];break;case K.ChooseKeyPassphrase:t="mx_CreateSecretStorageDialog_centeredTitle"}return r.createElement(C.A,{className:this.classNames,onFinished:this.props.onFinished,top:this.topComponent,title:this.titleForPhase(this.state.phase),titleClass:t,hasCancel:this.props.hasCancel&&[K.Passphrase].includes(this.state.phase),fixedWidth:!1},r.createElement("div",null,e))}}(0,a.A)(D,"defaultProps",{hasCancel:!0,forceReset:!1,resetCrossSigning:!1})},"./src/components/views/auth/PassphraseConfirmField.tsx":(e,t,s)=>{"use strict";s.d(t,{A:()=>l});var a=s("./node_modules/@babel/runtime/helpers/esm/defineProperty.js"),r=s("./node_modules/react/index.js"),n=s("./src/components/views/elements/Field.tsx"),o=s("./src/components/views/elements/Validation.tsx"),i=s("./src/languageHandler.tsx");class c extends r.PureComponent{constructor(...e){super(...e),(0,a.A)(this,"validate",(0,o.A)({rules:[{key:"required",test:({value:e,allowEmpty:t})=>t||!!e,invalid:()=>(0,i._t)(this.props.labelRequired)},{key:"match",test:({value:e})=>!e||e===this.props.password,invalid:()=>(0,i._t)(this.props.labelInvalid)}]})),(0,a.A)(this,"onValidate",(async e=>{const t=await this.validate(e);return this.props.onValidate&&this.props.onValidate(t),t}))}render(){return r.createElement(n.A,{id:this.props.id,ref:this.props.fieldRef,type:"password",label:(0,i._t)(this.props.label),autoComplete:this.props.autoComplete,value:this.props.value,onChange:this.props.onChange,onValidate:this.onValidate,autoFocus:this.props.autoFocus,tooltipAlignment:this.props.tooltipAlignment})}}(0,a.A)(c,"defaultProps",{label:(0,i.AO)("auth|change_password_confirm_label"),labelRequired:(0,i.AO)("auth|change_password_confirm_label"),labelInvalid:(0,i.AO)("auth|change_password_confirm_invalid")});const l=c},"./src/components/views/auth/PassphraseField.tsx":(e,t,s)=>{"use strict";s.d(t,{A:()=>g});var a,r=s("./node_modules/@babel/runtime/helpers/esm/defineProperty.js"),n=s("./node_modules/react/index.js"),o=s("./node_modules/classnames/index.js"),i=s.n(o),c=s("./src/SdkConfig.ts"),l=s("./src/components/views/elements/Validation.tsx"),p=s("./src/languageHandler.tsx"),u=s("./src/components/views/elements/Field.tsx"),h=s("./src/MatrixClientPeg.ts"),d=s("./src/tchap/util/TchapApi.ts"),m=s("./src/tchap/util/TchapUtils.ts");class _{static async getRules(){if(!this.passwordRules){const e=m.A.randomHomeServer().base_url,t=await fetch(`${e}${d.A.passwordRulesUrl}`);this.passwordRules=await t.json()}return this.passwordRules}static minimumLength(e,t){return e.length>=t}static requireUppercase(e,t){return!t||/[A-Z]/.test(e)}static requireSymbol(e,t){return!t||/[^a-zA-Z0-9]/.test(e)}static requireDigit(e,t){return!t||/[0-9]/.test(e)}static requireLowercase(e,t){return!t||/[a-z]/.test(e)}}a=_,(0,r.A)(_,"passwordRules",null),(0,r.A)(_,"validate",(async e=>{const t=e.value,s=await a.getRules(),r=Object.entries(s).reduce(((e,[s,r])=>{const n=((e,s)=>{switch(e){case"m.minimum_length":return a.minimumLength(t,s)?"":(0,p._t)("a minimum of %(number)s characters",{number:s});case"m.require_digit":return a.requireDigit(t,s)?"":(0,p._t)("a number");case"m.require_symbol":return a.requireSymbol(t,s)?"":(0,p._t)("a symbol");case"m.require_lowercase":return a.requireLowercase(t,s)?"":(0,p._t)("a lowercase letter");case"m.require_uppercase":return a.requireUppercase(t,s)?"":(0,p._t)("an uppercase letter");default:throw new Error("Unknown password rule : "+e)}})(s,r);return n.length&&e.push(n),e}),[]);return 0===r.length?{valid:!0}:{valid:!1,feedback:n.createElement("div",{className:"mx_Validation mx_Validation_invalid"},n.createElement("div",{className:"mx_Validation_description mx_Validation_invalid"},(0,p._t)("Your password must include:")),n.createElement("ul",{className:"mx_Validation_details"},r.map((e=>n.createElement("li",{className:"mx_Validation_detail mx_Validation_invalid"},e)))))}}));class y extends n.PureComponent{constructor(...e){super(...e),(0,r.A)(this,"validate",(0,l.A)({description:function(e){const t=e?e.score:0;return n.createElement("progress",{className:"mx_PassphraseField_progress",max:4,value:t})},deriveData:async({value:e})=>{if(!e)return null;const{scorePassword:t}=await Promise.all([s.e(6501),s.e(9393)]).then(s.bind(s,"./src/utils/PasswordScorer.ts"));return t(h.J.get(),e,this.props.userInputs)},rules:[{key:"required",test:({value:e,allowEmpty:t})=>t||!!e,invalid:()=>(0,p._t)(this.props.labelEnterPassword)},{key:"complexity",test:async function({value:e},t){if(!e||!t)return!1;const s=t.score>=this.props.minScore;return c.Ay.get("dangerously_allow_unsafe_and_insecure_passwords")||s},valid:function(e){return e&&e.score>=this.props.minScore?(0,p._t)(this.props.labelStrongPassword):(0,p._t)(this.props.labelAllowedButUnsafe)},invalid:function(e){if(!e)return null;const{feedback:t}=e;return t.warning||t.suggestions[0]||(0,p._t)("auth|password_field_keep_going_prompt")}}],memoize:!0})),(0,r.A)(this,"onValidate",(async e=>{const t=await _.validate(e);return this.props.onValidate&&this.props.onValidate(t),t}))}render(){return n.createElement(u.A,{id:this.props.id,autoFocus:this.props.autoFocus,className:i()("mx_PassphraseField",this.props.className),ref:this.props.fieldRef,type:"password",autoComplete:"new-password",label:(0,p._t)(this.props.label),value:this.props.value,onChange:this.props.onChange,onValidate:this.onValidate,tooltipAlignment:this.props.tooltipAlignment})}}(0,r.A)(y,"defaultProps",{label:(0,p.AO)("common|password"),labelEnterPassword:(0,p.AO)("auth|password_field_label"),labelStrongPassword:(0,p.AO)("auth|password_field_strong_label"),labelAllowedButUnsafe:(0,p.AO)("auth|password_field_weak_label")});const g=y},"./src/utils/device/dehydration.ts":(e,t,s)=>{"use strict";s.d(t,{b:()=>n});var a=s("./node_modules/matrix-js-sdk/src/logger.ts"),r=s("./src/MatrixClientPeg.ts");async function n(e=!1){const t=r.J.safeGet().getCrypto();await async function(e){if(!e)return!1;if(!await e.isDehydrationSupported())return!1;const t=await r.J.safeGet().waitForClientWellKnown();return!(null==t||!t["org.matrix.msc3814"])}(t)&&(a.v.log("Device dehydration enabled"),await t.startDehydration(e))}},"./node_modules/file-saver/dist/FileSaver.min.js":function(e,t,s){var a,r,n;r=[],void 0===(n="function"==typeof(a=function(){"use strict";function t(e,t){return void 0===t?t={autoBom:!1}:"object"!=typeof t&&(console.warn("Deprecated: Expected third argument to be a object"),t={autoBom:!t}),t.autoBom&&/^\s*(?:text\/\S*|application\/xml|\S*\/\S*\+xml)\s*;.*charset\s*=\s*utf-8/i.test(e.type)?new Blob(["\ufeff",e],{type:e.type}):e}function a(e,t,s){var a=new XMLHttpRequest;a.open("GET",e),a.responseType="blob",a.onload=function(){c(a.response,t,s)},a.onerror=function(){console.error("could not download file")},a.send()}function r(e){var t=new XMLHttpRequest;t.open("HEAD",e,!1);try{t.send()}catch(e){}return 200<=t.status&&299>=t.status}function n(e){try{e.dispatchEvent(new MouseEvent("click"))}catch(s){var t=document.createEvent("MouseEvents");t.initMouseEvent("click",!0,!0,window,0,0,0,80,20,!1,!1,!1,!1,0,null),e.dispatchEvent(t)}}var o="object"==typeof window&&window.window===window?window:"object"==typeof self&&self.self===self?self:"object"==typeof s.g&&s.g.global===s.g?s.g:void 0,i=o.navigator&&/Macintosh/.test(navigator.userAgent)&&/AppleWebKit/.test(navigator.userAgent)&&!/Safari/.test(navigator.userAgent),c=o.saveAs||("object"!=typeof window||window!==o?function(){}:"download"in HTMLAnchorElement.prototype&&!i?function(e,t,s){var i=o.URL||o.webkitURL,c=document.createElement("a");t=t||e.name||"download",c.download=t,c.rel="noopener","string"==typeof e?(c.href=e,c.origin===location.origin?n(c):r(c.href)?a(e,t,s):n(c,c.target="_blank")):(c.href=i.createObjectURL(e),setTimeout((function(){i.revokeObjectURL(c.href)}),4e4),setTimeout((function(){n(c)}),0))}:"msSaveOrOpenBlob"in navigator?function(e,s,o){if(s=s||e.name||"download","string"!=typeof e)navigator.msSaveOrOpenBlob(t(e,o),s);else if(r(e))a(e,s,o);else{var i=document.createElement("a");i.href=e,i.target="_blank",setTimeout((function(){n(i)}))}}:function(e,t,s,r){if((r=r||open("","_blank"))&&(r.document.title=r.document.body.innerText="downloading..."),"string"==typeof e)return a(e,t,s);var n="application/octet-stream"===e.type,c=/constructor/i.test(o.HTMLElement)||o.safari,l=/CriOS\/[\d]+/.test(navigator.userAgent);if((l||n&&c||i)&&"undefined"!=typeof FileReader){var p=new FileReader;p.onloadend=function(){var e=p.result;e=l?e:e.replace(/^data:[^;]*;/,"data:attachment/file;"),r?r.location.href=e:location=e,r=null},p.readAsDataURL(e)}else{var u=o.URL||o.webkitURL,h=u.createObjectURL(e);r?r.location=h:location.href=h,r=null,setTimeout((function(){u.revokeObjectURL(h)}),4e4)}});o.saveAs=c.saveAs=c,e.exports=c})?a.apply(t,r):a)||(e.exports=n)}}]);
//# sourceMappingURL=1127.js.map