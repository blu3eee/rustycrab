// Auto-generated from ChromeDevTools/devtools-protocol at commit 15f524c8f5ce5b317ddcdf5e6f875d6eb8bdac88
#[allow(unused)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod cdp {
    pub mod types {
        use serde::{Deserialize, Serialize};
        use std::fmt::Debug;
        pub type JsFloat = f64;
        pub type JsUInt = u32;
        pub type WindowId = JsUInt;
        pub type CallId = JsUInt;
        #[derive(Serialize, Debug)]
        pub struct MethodCall<T>
        where
            T: Debug,
        {
            #[serde(rename = "method")]
            method_name: &'static str,
            pub id: CallId,
            params: T,
        }
        impl<T> MethodCall<T>
        where
            T: Debug,
        {
            pub fn get_params(&self) -> &T {
                &self.params
            }
        }
        pub trait Method: Debug {
            const NAME: &'static str;
            type ReturnObject: serde::de::DeserializeOwned + std::fmt::Debug;
            fn to_method_call(self, call_id: CallId) -> MethodCall<Self>
            where
                Self: std::marker::Sized,
            {
                MethodCall {
                    id: call_id,
                    params: self,
                    method_name: Self::NAME,
                }
            }
        }
        #[derive(Deserialize, Debug, Clone, PartialEq)]
        #[serde(tag = "method")]
        #[allow(clippy::large_enum_variant)]
        pub enum Event {
            #[serde(rename = "Accessibility.loadComplete")]
            AccessibilityLoadComplete(super::Accessibility::events::LoadCompleteEvent),
            #[serde(rename = "Accessibility.nodesUpdated")]
            AccessibilityNodesUpdated(super::Accessibility::events::NodesUpdatedEvent),
            #[serde(rename = "Animation.animationCanceled")]
            AnimationCanceled(super::Animation::events::AnimationCanceledEvent),
            #[serde(rename = "Animation.animationCreated")]
            AnimationCreated(super::Animation::events::AnimationCreatedEvent),
            #[serde(rename = "Animation.animationStarted")]
            AnimationStarted(super::Animation::events::AnimationStartedEvent),
            #[serde(rename = "Audits.issueAdded")]
            AuditsIssueAdded(super::Audits::events::IssueAddedEvent),
            #[serde(rename = "BackgroundService.recordingStateChanged")]
            BackgroundServiceRecordingStateChanged(
                super::BackgroundService::events::RecordingStateChangedEvent,
            ),
            #[serde(rename = "BackgroundService.backgroundServiceEventReceived")]
            BackgroundServiceEventReceived(
                super::BackgroundService::events::BackgroundServiceEventReceivedEvent,
            ),
            #[serde(rename = "Browser.downloadWillBegin")]
            BrowserDownloadWillBegin(super::Browser::events::DownloadWillBeginEvent),
            #[serde(rename = "Browser.downloadProgress")]
            BrowserDownloadProgress(super::Browser::events::DownloadProgressEvent),
            #[serde(rename = "CSS.fontsUpdated")]
            CSSFontsUpdated(super::CSS::events::FontsUpdatedEvent),
            #[serde(rename = "CSS.mediaQueryResultChanged")]
            CSSMediaQueryResultChanged(super::CSS::events::MediaQueryResultChangedEvent),
            #[serde(rename = "CSS.styleSheetAdded")]
            CSSStyleSheetAdded(super::CSS::events::StyleSheetAddedEvent),
            #[serde(rename = "CSS.styleSheetChanged")]
            CSSStyleSheetChanged(super::CSS::events::StyleSheetChangedEvent),
            #[serde(rename = "CSS.styleSheetRemoved")]
            CSSStyleSheetRemoved(super::CSS::events::StyleSheetRemovedEvent),
            #[serde(rename = "Cast.sinksUpdated")]
            CastSinksUpdated(super::Cast::events::SinksUpdatedEvent),
            #[serde(rename = "Cast.issueUpdated")]
            CastIssueUpdated(super::Cast::events::IssueUpdatedEvent),
            #[serde(rename = "DOM.attributeModified")]
            DOMAttributeModified(super::DOM::events::AttributeModifiedEvent),
            #[serde(rename = "DOM.attributeRemoved")]
            DOMAttributeRemoved(super::DOM::events::AttributeRemovedEvent),
            #[serde(rename = "DOM.characterDataModified")]
            DOMCharacterDataModified(super::DOM::events::CharacterDataModifiedEvent),
            #[serde(rename = "DOM.childNodeCountUpdated")]
            DOMChildNodeCountUpdated(super::DOM::events::ChildNodeCountUpdatedEvent),
            #[serde(rename = "DOM.childNodeInserted")]
            DOMChildNodeInserted(super::DOM::events::ChildNodeInsertedEvent),
            #[serde(rename = "DOM.childNodeRemoved")]
            DOMChildNodeRemoved(super::DOM::events::ChildNodeRemovedEvent),
            #[serde(rename = "DOM.distributedNodesUpdated")]
            DOMDistributedNodesUpdated(super::DOM::events::DistributedNodesUpdatedEvent),
            #[serde(rename = "DOM.documentUpdated")]
            DOMDocumentUpdated(super::DOM::events::DocumentUpdatedEvent),
            #[serde(rename = "DOM.inlineStyleInvalidated")]
            DOMInlineStyleInvalidated(super::DOM::events::InlineStyleInvalidatedEvent),
            #[serde(rename = "DOM.pseudoElementAdded")]
            DOMPseudoElementAdded(super::DOM::events::PseudoElementAddedEvent),
            #[serde(rename = "DOM.pseudoElementRemoved")]
            DOMPseudoElementRemoved(super::DOM::events::PseudoElementRemovedEvent),
            #[serde(rename = "DOM.setChildNodes")]
            DOMSetChildNodes(super::DOM::events::SetChildNodesEvent),
            #[serde(rename = "DOM.shadowRootPopped")]
            DOMShadowRootPopped(super::DOM::events::ShadowRootPoppedEvent),
            #[serde(rename = "DOM.shadowRootPushed")]
            DOMShadowRootPushed(super::DOM::events::ShadowRootPushedEvent),
            #[serde(rename = "DOMStorage.domStorageItemAdded")]
            DOMStorageDomStorageItemAdded(super::DOMStorage::events::DomStorageItemAddedEvent),
            #[serde(rename = "DOMStorage.domStorageItemRemoved")]
            DOMStorageDomStorageItemRemoved(super::DOMStorage::events::DomStorageItemRemovedEvent),
            #[serde(rename = "DOMStorage.domStorageItemUpdated")]
            DOMStorageDomStorageItemUpdated(super::DOMStorage::events::DomStorageItemUpdatedEvent),
            #[serde(rename = "DOMStorage.domStorageItemsCleared")]
            DOMStorageDomStorageItemsCleared(
                super::DOMStorage::events::DomStorageItemsClearedEvent,
            ),
            #[serde(rename = "Database.addDatabase")]
            AddDatabase(super::Database::events::AddDatabaseEvent),
            #[serde(rename = "Emulation.virtualTimeBudgetExpired")]
            EmulationVirtualTimeBudgetExpired(
                super::Emulation::events::VirtualTimeBudgetExpiredEvent,
            ),
            #[serde(rename = "HeadlessExperimental.needsBeginFramesChanged")]
            HeadlessExperimentalNeedsBeginFramesChanged(
                super::HeadlessExperimental::events::NeedsBeginFramesChangedEvent,
            ),
            #[serde(rename = "Input.dragIntercepted")]
            InputDragIntercepted(super::Input::events::DragInterceptedEvent),
            #[serde(rename = "Inspector.detached")]
            InspectorDetached(super::Inspector::events::DetachedEvent),
            #[serde(rename = "Inspector.targetCrashed")]
            InspectorTargetCrashed(super::Inspector::events::TargetCrashedEvent),
            #[serde(rename = "Inspector.targetReloadedAfterCrash")]
            InspectorTargetReloadedAfterCrash(
                super::Inspector::events::TargetReloadedAfterCrashEvent,
            ),
            #[serde(rename = "LayerTree.layerPainted")]
            LayerTreeLayerPainted(super::LayerTree::events::LayerPaintedEvent),
            #[serde(rename = "LayerTree.layerTreeDidChange")]
            LayerTreeDidChange(super::LayerTree::events::LayerTreeDidChangeEvent),
            #[serde(rename = "Log.entryAdded")]
            LogEntryAdded(super::Log::events::EntryAddedEvent),
            #[serde(rename = "Network.dataReceived")]
            NetworkDataReceived(super::Network::events::DataReceivedEvent),
            #[serde(rename = "Network.eventSourceMessageReceived")]
            NetworkEventSourceMessageReceived(
                super::Network::events::EventSourceMessageReceivedEvent,
            ),
            #[serde(rename = "Network.loadingFailed")]
            NetworkLoadingFailed(super::Network::events::LoadingFailedEvent),
            #[serde(rename = "Network.loadingFinished")]
            NetworkLoadingFinished(super::Network::events::LoadingFinishedEvent),
            #[serde(rename = "Network.requestIntercepted")]
            NetworkRequestIntercepted(super::Network::events::RequestInterceptedEvent),
            #[serde(rename = "Network.requestServedFromCache")]
            NetworkRequestServedFromCache(super::Network::events::RequestServedFromCacheEvent),
            #[serde(rename = "Network.requestWillBeSent")]
            NetworkRequestWillBeSent(super::Network::events::RequestWillBeSentEvent),
            #[serde(rename = "Network.resourceChangedPriority")]
            NetworkResourceChangedPriority(super::Network::events::ResourceChangedPriorityEvent),
            #[serde(rename = "Network.signedExchangeReceived")]
            NetworkSignedExchangeReceived(super::Network::events::SignedExchangeReceivedEvent),
            #[serde(rename = "Network.responseReceived")]
            NetworkResponseReceived(super::Network::events::ResponseReceivedEvent),
            #[serde(rename = "Network.webSocketClosed")]
            NetworkWebSocketClosed(super::Network::events::WebSocketClosedEvent),
            #[serde(rename = "Network.webSocketCreated")]
            NetworkWebSocketCreated(super::Network::events::WebSocketCreatedEvent),
            #[serde(rename = "Network.webSocketFrameError")]
            NetworkWebSocketFrameError(super::Network::events::WebSocketFrameErrorEvent),
            #[serde(rename = "Network.webSocketFrameReceived")]
            NetworkWebSocketFrameReceived(super::Network::events::WebSocketFrameReceivedEvent),
            #[serde(rename = "Network.webSocketFrameSent")]
            NetworkWebSocketFrameSent(super::Network::events::WebSocketFrameSentEvent),
            #[serde(rename = "Network.webSocketHandshakeResponseReceived")]
            NetworkWebSocketHandshakeResponseReceived(
                super::Network::events::WebSocketHandshakeResponseReceivedEvent,
            ),
            #[serde(rename = "Network.webSocketWillSendHandshakeRequest")]
            NetworkWebSocketWillSendHandshakeRequest(
                super::Network::events::WebSocketWillSendHandshakeRequestEvent,
            ),
            #[serde(rename = "Network.webTransportCreated")]
            NetworkWebTransportCreated(super::Network::events::WebTransportCreatedEvent),
            #[serde(rename = "Network.webTransportConnectionEstablished")]
            NetworkWebTransportConnectionEstablished(
                super::Network::events::WebTransportConnectionEstablishedEvent,
            ),
            #[serde(rename = "Network.webTransportClosed")]
            NetworkWebTransportClosed(super::Network::events::WebTransportClosedEvent),
            #[serde(rename = "Network.requestWillBeSentExtraInfo")]
            NetworkRequestWillBeSentExtraInfo(
                super::Network::events::RequestWillBeSentExtraInfoEvent,
            ),
            #[serde(rename = "Network.responseReceivedExtraInfo")]
            NetworkResponseReceivedExtraInfo(
                super::Network::events::ResponseReceivedExtraInfoEvent,
            ),
            #[serde(rename = "Network.trustTokenOperationDone")]
            NetworkTrustTokenOperationDone(super::Network::events::TrustTokenOperationDoneEvent),
            #[serde(rename = "Network.subresourceWebBundleMetadataReceived")]
            NetworkSubresourceWebBundleMetadataReceived(
                super::Network::events::SubresourceWebBundleMetadataReceivedEvent,
            ),
            #[serde(rename = "Network.subresourceWebBundleMetadataError")]
            NetworkSubresourceWebBundleMetadataError(
                super::Network::events::SubresourceWebBundleMetadataErrorEvent,
            ),
            #[serde(rename = "Network.subresourceWebBundleInnerResponseParsed")]
            NetworkSubresourceWebBundleInnerResponseParsed(
                super::Network::events::SubresourceWebBundleInnerResponseParsedEvent,
            ),
            #[serde(rename = "Network.subresourceWebBundleInnerResponseError")]
            NetworkSubresourceWebBundleInnerResponseError(
                super::Network::events::SubresourceWebBundleInnerResponseErrorEvent,
            ),
            #[serde(rename = "Network.reportingApiReportAdded")]
            NetworkReportingApiReportAdded(super::Network::events::ReportingApiReportAddedEvent),
            #[serde(rename = "Network.reportingApiReportUpdated")]
            NetworkReportingApiReportUpdated(
                super::Network::events::ReportingApiReportUpdatedEvent,
            ),
            #[serde(rename = "Network.reportingApiEndpointsChangedForOrigin")]
            NetworkReportingApiEndpointsChangedForOrigin(
                super::Network::events::ReportingApiEndpointsChangedForOriginEvent,
            ),
            #[serde(rename = "Overlay.inspectNodeRequested")]
            OverlayInspectNodeRequested(super::Overlay::events::InspectNodeRequestedEvent),
            #[serde(rename = "Overlay.nodeHighlightRequested")]
            OverlayNodeHighlightRequested(super::Overlay::events::NodeHighlightRequestedEvent),
            #[serde(rename = "Overlay.screenshotRequested")]
            OverlayScreenshotRequested(super::Overlay::events::ScreenshotRequestedEvent),
            #[serde(rename = "Overlay.inspectModeCanceled")]
            OverlayInspectModeCanceled(super::Overlay::events::InspectModeCanceledEvent),
            #[serde(rename = "Page.domContentEventFired")]
            PageDomContentEventFired(super::Page::events::DomContentEventFiredEvent),
            #[serde(rename = "Page.fileChooserOpened")]
            PageFileChooserOpened(super::Page::events::FileChooserOpenedEvent),
            #[serde(rename = "Page.frameAttached")]
            PageFrameAttached(super::Page::events::FrameAttachedEvent),
            #[serde(rename = "Page.frameClearedScheduledNavigation")]
            PageFrameClearedScheduledNavigation(
                super::Page::events::FrameClearedScheduledNavigationEvent,
            ),
            #[serde(rename = "Page.frameDetached")]
            PageFrameDetached(super::Page::events::FrameDetachedEvent),
            #[serde(rename = "Page.frameNavigated")]
            PageFrameNavigated(super::Page::events::FrameNavigatedEvent),
            #[serde(rename = "Page.documentOpened")]
            PageDocumentOpened(super::Page::events::DocumentOpenedEvent),
            #[serde(rename = "Page.frameResized")]
            PageFrameResized(super::Page::events::FrameResizedEvent),
            #[serde(rename = "Page.frameRequestedNavigation")]
            PageFrameRequestedNavigation(super::Page::events::FrameRequestedNavigationEvent),
            #[serde(rename = "Page.frameScheduledNavigation")]
            PageFrameScheduledNavigation(super::Page::events::FrameScheduledNavigationEvent),
            #[serde(rename = "Page.frameStartedLoading")]
            PageFrameStartedLoading(super::Page::events::FrameStartedLoadingEvent),
            #[serde(rename = "Page.frameStoppedLoading")]
            PageFrameStoppedLoading(super::Page::events::FrameStoppedLoadingEvent),
            #[serde(rename = "Page.downloadWillBegin")]
            PageDownloadWillBegin(super::Page::events::DownloadWillBeginEvent),
            #[serde(rename = "Page.downloadProgress")]
            PageDownloadProgress(super::Page::events::DownloadProgressEvent),
            #[serde(rename = "Page.interstitialHidden")]
            PageInterstitialHidden(super::Page::events::InterstitialHiddenEvent),
            #[serde(rename = "Page.interstitialShown")]
            PageInterstitialShown(super::Page::events::InterstitialShownEvent),
            #[serde(rename = "Page.javascriptDialogClosed")]
            PageJavascriptDialogClosed(super::Page::events::JavascriptDialogClosedEvent),
            #[serde(rename = "Page.javascriptDialogOpening")]
            PageJavascriptDialogOpening(super::Page::events::JavascriptDialogOpeningEvent),
            #[serde(rename = "Page.lifecycleEvent")]
            PageLifecycleEvent(super::Page::events::LifecycleEventEvent),
            #[serde(rename = "Page.backForwardCacheNotUsed")]
            PageBackForwardCacheNotUsed(super::Page::events::BackForwardCacheNotUsedEvent),
            #[serde(rename = "Page.loadEventFired")]
            PageLoadEventFired(super::Page::events::LoadEventFiredEvent),
            #[serde(rename = "Page.navigatedWithinDocument")]
            PageNavigatedWithinDocument(super::Page::events::NavigatedWithinDocumentEvent),
            #[serde(rename = "Page.screencastFrame")]
            PageScreencastFrame(super::Page::events::ScreencastFrameEvent),
            #[serde(rename = "Page.screencastVisibilityChanged")]
            PageScreencastVisibilityChanged(super::Page::events::ScreencastVisibilityChangedEvent),
            #[serde(rename = "Page.windowOpen")]
            PageWindowOpen(super::Page::events::WindowOpenEvent),
            #[serde(rename = "Page.compilationCacheProduced")]
            PageCompilationCacheProduced(super::Page::events::CompilationCacheProducedEvent),
            #[serde(rename = "Performance.metrics")]
            PerformanceMetrics(super::Performance::events::MetricsEvent),
            #[serde(rename = "PerformanceTimeline.timelineEventAdded")]
            PerformanceTimelineTimelineEventAdded(
                super::PerformanceTimeline::events::TimelineEventAddedEvent,
            ),
            #[serde(rename = "Security.certificateError")]
            SecurityCertificateError(super::Security::events::CertificateErrorEvent),
            #[serde(rename = "Security.visibleSecurityStateChanged")]
            VisibleSecurityStateChanged(super::Security::events::VisibleSecurityStateChangedEvent),
            #[serde(rename = "Security.securityStateChanged")]
            SecurityStateChanged(super::Security::events::SecurityStateChangedEvent),
            #[serde(rename = "ServiceWorker.workerErrorReported")]
            ServiceWorkerWorkerErrorReported(
                super::ServiceWorker::events::WorkerErrorReportedEvent,
            ),
            #[serde(rename = "ServiceWorker.workerRegistrationUpdated")]
            ServiceWorkerWorkerRegistrationUpdated(
                super::ServiceWorker::events::WorkerRegistrationUpdatedEvent,
            ),
            #[serde(rename = "ServiceWorker.workerVersionUpdated")]
            ServiceWorkerWorkerVersionUpdated(
                super::ServiceWorker::events::WorkerVersionUpdatedEvent,
            ),
            #[serde(rename = "Storage.cacheStorageContentUpdated")]
            CacheStorageContentUpdated(super::Storage::events::CacheStorageContentUpdatedEvent),
            #[serde(rename = "Storage.cacheStorageListUpdated")]
            CacheStorageListUpdated(super::Storage::events::CacheStorageListUpdatedEvent),
            #[serde(rename = "Storage.indexedDBContentUpdated")]
            StorageIndexedDBContentUpdated(super::Storage::events::IndexedDBContentUpdatedEvent),
            #[serde(rename = "Storage.indexedDBListUpdated")]
            StorageIndexedDBListUpdated(super::Storage::events::IndexedDBListUpdatedEvent),
            #[serde(rename = "Target.attachedToTarget")]
            AttachedToTarget(super::Target::events::AttachedToTargetEvent),
            #[serde(rename = "Target.detachedFromTarget")]
            DetachedFromTarget(super::Target::events::DetachedFromTargetEvent),
            #[serde(rename = "Target.receivedMessageFromTarget")]
            ReceivedMessageFromTarget(super::Target::events::ReceivedMessageFromTargetEvent),
            #[serde(rename = "Target.targetCreated")]
            TargetCreated(super::Target::events::TargetCreatedEvent),
            #[serde(rename = "Target.targetDestroyed")]
            TargetDestroyed(super::Target::events::TargetDestroyedEvent),
            #[serde(rename = "Target.targetCrashed")]
            TargetCrashed(super::Target::events::TargetCrashedEvent),
            #[serde(rename = "Target.targetInfoChanged")]
            TargetInfoChanged(super::Target::events::TargetInfoChangedEvent),
            #[serde(rename = "Tethering.accepted")]
            TetheringAccepted(super::Tethering::events::AcceptedEvent),
            #[serde(rename = "Tracing.bufferUsage")]
            TracingBufferUsage(super::Tracing::events::BufferUsageEvent),
            #[serde(rename = "Tracing.dataCollected")]
            TracingDataCollected(super::Tracing::events::DataCollectedEvent),
            #[serde(rename = "Tracing.tracingComplete")]
            TracingComplete(super::Tracing::events::TracingCompleteEvent),
            #[serde(rename = "Fetch.requestPaused")]
            FetchRequestPaused(super::Fetch::events::RequestPausedEvent),
            #[serde(rename = "Fetch.authRequired")]
            FetchAuthRequired(super::Fetch::events::AuthRequiredEvent),
            #[serde(rename = "WebAudio.contextCreated")]
            WebAudioContextCreated(super::WebAudio::events::ContextCreatedEvent),
            #[serde(rename = "WebAudio.contextWillBeDestroyed")]
            WebAudioContextWillBeDestroyed(super::WebAudio::events::ContextWillBeDestroyedEvent),
            #[serde(rename = "WebAudio.contextChanged")]
            WebAudioContextChanged(super::WebAudio::events::ContextChangedEvent),
            #[serde(rename = "WebAudio.audioListenerCreated")]
            WebAudioAudioListenerCreated(super::WebAudio::events::AudioListenerCreatedEvent),
            #[serde(rename = "WebAudio.audioListenerWillBeDestroyed")]
            WebAudioAudioListenerWillBeDestroyed(
                super::WebAudio::events::AudioListenerWillBeDestroyedEvent,
            ),
            #[serde(rename = "WebAudio.audioNodeCreated")]
            WebAudioAudioNodeCreated(super::WebAudio::events::AudioNodeCreatedEvent),
            #[serde(rename = "WebAudio.audioNodeWillBeDestroyed")]
            WebAudioAudioNodeWillBeDestroyed(
                super::WebAudio::events::AudioNodeWillBeDestroyedEvent,
            ),
            #[serde(rename = "WebAudio.audioParamCreated")]
            WebAudioAudioParamCreated(super::WebAudio::events::AudioParamCreatedEvent),
            #[serde(rename = "WebAudio.audioParamWillBeDestroyed")]
            WebAudioAudioParamWillBeDestroyed(
                super::WebAudio::events::AudioParamWillBeDestroyedEvent,
            ),
            #[serde(rename = "WebAudio.nodesConnected")]
            WebAudioNodesConnected(super::WebAudio::events::NodesConnectedEvent),
            #[serde(rename = "WebAudio.nodesDisconnected")]
            WebAudioNodesDisconnected(super::WebAudio::events::NodesDisconnectedEvent),
            #[serde(rename = "WebAudio.nodeParamConnected")]
            WebAudioNodeParamConnected(super::WebAudio::events::NodeParamConnectedEvent),
            #[serde(rename = "WebAudio.nodeParamDisconnected")]
            WebAudioNodeParamDisconnected(super::WebAudio::events::NodeParamDisconnectedEvent),
            #[serde(rename = "Media.playerPropertiesChanged")]
            MediaPlayerPropertiesChanged(super::Media::events::PlayerPropertiesChangedEvent),
            #[serde(rename = "Media.playerEventsAdded")]
            MediaPlayerEventsAdded(super::Media::events::PlayerEventsAddedEvent),
            #[serde(rename = "Media.playerMessagesLogged")]
            MediaPlayerMessagesLogged(super::Media::events::PlayerMessagesLoggedEvent),
            #[serde(rename = "Media.playerErrorsRaised")]
            MediaPlayerErrorsRaised(super::Media::events::PlayerErrorsRaisedEvent),
            #[serde(rename = "Media.playersCreated")]
            MediaPlayersCreated(super::Media::events::PlayersCreatedEvent),
            #[serde(rename = "Console.messageAdded")]
            ConsoleMessageAdded(super::Console::events::MessageAddedEvent),
            #[serde(rename = "Debugger.breakpointResolved")]
            DebuggerBreakpointResolved(super::Debugger::events::BreakpointResolvedEvent),
            #[serde(rename = "Debugger.paused")]
            DebuggerPaused(super::Debugger::events::PausedEvent),
            #[serde(rename = "Debugger.resumed")]
            DebuggerResumed(super::Debugger::events::ResumedEvent),
            #[serde(rename = "Debugger.scriptFailedToParse")]
            DebuggerScriptFailedToParse(super::Debugger::events::ScriptFailedToParseEvent),
            #[serde(rename = "Debugger.scriptParsed")]
            DebuggerScriptParsed(super::Debugger::events::ScriptParsedEvent),
            #[serde(rename = "HeapProfiler.addHeapSnapshotChunk")]
            HeapProfilerAddHeapSnapshotChunk(
                super::HeapProfiler::events::AddHeapSnapshotChunkEvent,
            ),
            #[serde(rename = "HeapProfiler.heapStatsUpdate")]
            HeapProfilerHeapStatsUpdate(super::HeapProfiler::events::HeapStatsUpdateEvent),
            #[serde(rename = "HeapProfiler.lastSeenObjectId")]
            HeapProfilerLastSeenObjectId(super::HeapProfiler::events::LastSeenObjectIdEvent),
            #[serde(rename = "HeapProfiler.reportHeapSnapshotProgress")]
            HeapProfilerReportHeapSnapshotProgress(
                super::HeapProfiler::events::ReportHeapSnapshotProgressEvent,
            ),
            #[serde(rename = "HeapProfiler.resetProfiles")]
            HeapProfilerResetProfiles(super::HeapProfiler::events::ResetProfilesEvent),
            #[serde(rename = "Profiler.consoleProfileFinished")]
            ProfilerConsoleProfileFinished(super::Profiler::events::ConsoleProfileFinishedEvent),
            #[serde(rename = "Profiler.consoleProfileStarted")]
            ProfilerConsoleProfileStarted(super::Profiler::events::ConsoleProfileStartedEvent),
            #[serde(rename = "Profiler.preciseCoverageDeltaUpdate")]
            ProfilerPreciseCoverageDeltaUpdate(
                super::Profiler::events::PreciseCoverageDeltaUpdateEvent,
            ),
            #[serde(rename = "Runtime.bindingCalled")]
            RuntimeBindingCalled(super::Runtime::events::BindingCalledEvent),
            #[serde(rename = "Runtime.consoleAPICalled")]
            RuntimeConsoleAPICalled(super::Runtime::events::ConsoleAPICalledEvent),
            #[serde(rename = "Runtime.exceptionRevoked")]
            RuntimeExceptionRevoked(super::Runtime::events::ExceptionRevokedEvent),
            #[serde(rename = "Runtime.exceptionThrown")]
            RuntimeExceptionThrown(super::Runtime::events::ExceptionThrownEvent),
            #[serde(rename = "Runtime.executionContextCreated")]
            RuntimeExecutionContextCreated(super::Runtime::events::ExecutionContextCreatedEvent),
            #[serde(rename = "Runtime.executionContextDestroyed")]
            RuntimeExecutionContextDestroyed(
                super::Runtime::events::ExecutionContextDestroyedEvent,
            ),
            #[serde(rename = "Runtime.executionContextsCleared")]
            RuntimeExecutionContextsCleared(super::Runtime::events::ExecutionContextsClearedEvent),
            #[serde(rename = "Runtime.inspectRequested")]
            RuntimeInspectRequested(super::Runtime::events::InspectRequestedEvent),
        }
    }
    pub mod Console {
        use super::types::*;
        use super::Runtime;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ConsoleMessageSource {
            #[serde(rename = "xml")]
            Xml,
            #[serde(rename = "javascript")]
            Javascript,
            #[serde(rename = "network")]
            Network,
            #[serde(rename = "console-api")]
            ConsoleApi,
            #[serde(rename = "storage")]
            Storage,
            #[serde(rename = "appcache")]
            Appcache,
            #[serde(rename = "rendering")]
            Rendering,
            #[serde(rename = "security")]
            Security,
            #[serde(rename = "other")]
            Other,
            #[serde(rename = "deprecation")]
            Deprecation,
            #[serde(rename = "worker")]
            Worker,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ConsoleMessageLevel {
            #[serde(rename = "log")]
            Log,
            #[serde(rename = "warning")]
            Warning,
            #[serde(rename = "error")]
            Error,
            #[serde(rename = "debug")]
            Debug,
            #[serde(rename = "info")]
            Info,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ConsoleMessage {
            #[serde(rename = "source")]
            pub source: ConsoleMessageSource,
            #[serde(rename = "level")]
            pub level: ConsoleMessageLevel,
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "line")]
            pub line: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "column")]
            pub column: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearMessages(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearMessagesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        impl Method for ClearMessages {
            const NAME: &'static str = "Console.clearMessages";
            type ReturnObject = ClearMessagesReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Console.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Console.enable";
            type ReturnObject = EnableReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct MessageAddedEvent {
                pub params: MessageAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct MessageAddedEventParams {
                #[serde(rename = "message")]
                pub message: super::ConsoleMessage,
            }
        }
    }
    pub mod Debugger {
        use super::types::*;
        use super::Runtime;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type BreakpointId = String;
        pub type CallFrameId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ScopeType {
            #[serde(rename = "global")]
            Global,
            #[serde(rename = "local")]
            Local,
            #[serde(rename = "with")]
            With,
            #[serde(rename = "closure")]
            Closure,
            #[serde(rename = "catch")]
            Catch,
            #[serde(rename = "block")]
            Block,
            #[serde(rename = "script")]
            Script,
            #[serde(rename = "eval")]
            Eval,
            #[serde(rename = "module")]
            Module,
            #[serde(rename = "wasm-expression-stack")]
            WasmExpressionStack,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum BreakLocationType {
            #[serde(rename = "debuggerStatement")]
            DebuggerStatement,
            #[serde(rename = "call")]
            Call,
            #[serde(rename = "return")]
            Return,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ScriptLanguage {
            #[serde(rename = "JavaScript")]
            JavaScript,
            #[serde(rename = "WebAssembly")]
            WebAssembly,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum DebugSymbolsType {
            #[serde(rename = "None")]
            None,
            #[serde(rename = "SourceMap")]
            SourceMap,
            #[serde(rename = "EmbeddedDWARF")]
            EmbeddedDwarf,
            #[serde(rename = "ExternalDWARF")]
            ExternalDwarf,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ContinueToLocationTarget_call_framesOption {
            #[serde(rename = "any")]
            Any,
            #[serde(rename = "current")]
            Current,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SetInstrumentationBreakpointInstrumentationOption {
            #[serde(rename = "beforeScriptExecution")]
            BeforeScriptExecution,
            #[serde(rename = "beforeScriptWithSourceMapExecution")]
            BeforeScriptWithSourceMapExecution,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SetPauseOnExceptionsStateOption {
            #[serde(rename = "none")]
            None,
            #[serde(rename = "uncaught")]
            Uncaught,
            #[serde(rename = "all")]
            All,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum PausedEventReasonOption {
            #[serde(rename = "ambiguous")]
            Ambiguous,
            #[serde(rename = "assert")]
            Assert,
            #[serde(rename = "CSPViolation")]
            CspViolation,
            #[serde(rename = "debugCommand")]
            DebugCommand,
            #[serde(rename = "DOM")]
            Dom,
            #[serde(rename = "EventListener")]
            EventListener,
            #[serde(rename = "exception")]
            Exception,
            #[serde(rename = "instrumentation")]
            Instrumentation,
            #[serde(rename = "OOM")]
            Oom,
            #[serde(rename = "other")]
            Other,
            #[serde(rename = "promiseRejection")]
            PromiseRejection,
            #[serde(rename = "XHR")]
            Xhr,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Location {
            #[serde(rename = "scriptId")]
            pub script_id: Runtime::ScriptId,
            #[serde(default)]
            #[serde(rename = "lineNumber")]
            pub line_number: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "columnNumber")]
            pub column_number: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ScriptPosition {
            #[serde(default)]
            #[serde(rename = "lineNumber")]
            pub line_number: JsUInt,
            #[serde(default)]
            #[serde(rename = "columnNumber")]
            pub column_number: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct LocationRange {
            #[serde(rename = "scriptId")]
            pub script_id: Runtime::ScriptId,
            #[serde(rename = "start")]
            pub start: ScriptPosition,
            #[serde(rename = "end")]
            pub end: ScriptPosition,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CallFrame {
            #[serde(rename = "callFrameId")]
            pub call_frame_id: CallFrameId,
            #[serde(default)]
            #[serde(rename = "functionName")]
            pub function_name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "functionLocation")]
            pub function_location: Option<Location>,
            #[serde(rename = "location")]
            pub location: Location,
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(rename = "scopeChain")]
            pub scope_chain: Vec<Scope>,
            #[serde(rename = "this")]
            pub this: Runtime::RemoteObject,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "returnValue")]
            pub return_value: Option<Runtime::RemoteObject>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Scope {
            #[serde(rename = "type")]
            pub Type: ScopeType,
            #[serde(rename = "object")]
            pub object: Runtime::RemoteObject,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "startLocation")]
            pub start_location: Option<Location>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "endLocation")]
            pub end_location: Option<Location>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SearchMatch {
            #[serde(default)]
            #[serde(rename = "lineNumber")]
            pub line_number: JsFloat,
            #[serde(default)]
            #[serde(rename = "lineContent")]
            pub line_content: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct BreakLocation {
            #[serde(rename = "scriptId")]
            pub script_id: Runtime::ScriptId,
            #[serde(default)]
            #[serde(rename = "lineNumber")]
            pub line_number: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "columnNumber")]
            pub column_number: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "type")]
            pub Type: Option<BreakLocationType>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DebugSymbols {
            #[serde(rename = "type")]
            pub Type: DebugSymbolsType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "externalURL")]
            pub external_url: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ContinueToLocation {
            #[serde(rename = "location")]
            pub location: Location,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "targetCallFrames")]
            pub target_call_frames: Option<ContinueToLocationTarget_call_framesOption>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Enable {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "maxScriptsCacheSize")]
            pub max_scripts_cache_size: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct EvaluateOnCallFrame {
            #[serde(rename = "callFrameId")]
            pub call_frame_id: CallFrameId,
            #[serde(default)]
            #[serde(rename = "expression")]
            pub expression: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "objectGroup")]
            pub object_group: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "includeCommandLineAPI")]
            pub include_command_line_api: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "silent")]
            pub silent: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "returnByValue")]
            pub return_by_value: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "generatePreview")]
            pub generate_preview: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "throwOnSideEffect")]
            pub throw_on_side_effect: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "timeout")]
            pub timeout: Option<Runtime::TimeDelta>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetPossibleBreakpoints {
            #[serde(rename = "start")]
            pub start: Location,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "end")]
            pub end: Option<Location>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "restrictToFunction")]
            pub restrict_to_function: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetScriptSource {
            #[serde(rename = "scriptId")]
            pub script_id: Runtime::ScriptId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetWasmBytecode {
            #[serde(rename = "scriptId")]
            pub script_id: Runtime::ScriptId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetStackTrace {
            #[serde(rename = "stackTraceId")]
            pub stack_trace_id: Runtime::StackTraceId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Pause(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PauseOnAsyncCall {
            #[serde(rename = "parentStackTraceId")]
            pub parent_stack_trace_id: Runtime::StackTraceId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RemoveBreakpoint {
            #[serde(rename = "breakpointId")]
            pub breakpoint_id: BreakpointId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RestartFrame {
            #[serde(rename = "callFrameId")]
            pub call_frame_id: CallFrameId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Resume {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "terminateOnResume")]
            pub terminate_on_resume: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SearchInContent {
            #[serde(rename = "scriptId")]
            pub script_id: Runtime::ScriptId,
            #[serde(default)]
            #[serde(rename = "query")]
            pub query: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "caseSensitive")]
            pub case_sensitive: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "isRegex")]
            pub is_regex: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetAsyncCallStackDepth {
            #[serde(default)]
            #[serde(rename = "maxDepth")]
            pub max_depth: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetBlackboxPatterns {
            #[serde(default)]
            #[serde(rename = "patterns")]
            pub patterns: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetBlackboxedRanges {
            #[serde(rename = "scriptId")]
            pub script_id: Runtime::ScriptId,
            #[serde(rename = "positions")]
            pub positions: Vec<ScriptPosition>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetBreakpoint {
            #[serde(rename = "location")]
            pub location: Location,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "condition")]
            pub condition: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetInstrumentationBreakpoint {
            #[serde(rename = "instrumentation")]
            pub instrumentation: SetInstrumentationBreakpointInstrumentationOption,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetBreakpointByUrl {
            #[serde(default)]
            #[serde(rename = "lineNumber")]
            pub line_number: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "urlRegex")]
            pub url_regex: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "scriptHash")]
            pub script_hash: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "columnNumber")]
            pub column_number: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "condition")]
            pub condition: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetBreakpointOnFunctionCall {
            #[serde(rename = "objectId")]
            pub object_id: Runtime::RemoteObjectId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "condition")]
            pub condition: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetBreakpointsActive {
            #[serde(default)]
            #[serde(rename = "active")]
            pub active: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetPauseOnExceptions {
            #[serde(rename = "state")]
            pub state: SetPauseOnExceptionsStateOption,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetReturnValue {
            #[serde(rename = "newValue")]
            pub new_value: Runtime::CallArgument,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetScriptSource {
            #[serde(rename = "scriptId")]
            pub script_id: Runtime::ScriptId,
            #[serde(default)]
            #[serde(rename = "scriptSource")]
            pub script_source: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "dryRun")]
            pub dry_run: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetSkipAllPauses {
            #[serde(default)]
            #[serde(rename = "skip")]
            pub skip: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetVariableValue {
            #[serde(default)]
            #[serde(rename = "scopeNumber")]
            pub scope_number: JsUInt,
            #[serde(default)]
            #[serde(rename = "variableName")]
            pub variable_name: String,
            #[serde(rename = "newValue")]
            pub new_value: Runtime::CallArgument,
            #[serde(rename = "callFrameId")]
            pub call_frame_id: CallFrameId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StepInto {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "breakOnAsyncCall")]
            pub break_on_async_call: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "skipList")]
            pub skip_list: Option<Vec<LocationRange>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StepOut(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StepOver {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "skipList")]
            pub skip_list: Option<Vec<LocationRange>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContinueToLocationReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct EnableReturnObject {
            #[serde(rename = "debuggerId")]
            pub debugger_id: Runtime::UniqueDebuggerId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct EvaluateOnCallFrameReturnObject {
            #[serde(rename = "result")]
            pub result: Runtime::RemoteObject,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "exceptionDetails")]
            pub exception_details: Option<Runtime::ExceptionDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetPossibleBreakpointsReturnObject {
            #[serde(rename = "locations")]
            pub locations: Vec<BreakLocation>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetScriptSourceReturnObject {
            #[serde(default)]
            #[serde(rename = "scriptSource")]
            pub script_source: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "bytecode")]
            pub bytecode: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetWasmBytecodeReturnObject {
            #[serde(default)]
            #[serde(rename = "bytecode")]
            pub bytecode: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetStackTraceReturnObject {
            #[serde(rename = "stackTrace")]
            pub stack_trace: Runtime::StackTrace,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PauseReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PauseOnAsyncCallReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RestartFrameReturnObject {
            #[serde(rename = "callFrames")]
            pub call_frames: Vec<CallFrame>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "asyncStackTrace")]
            pub async_stack_trace: Option<Runtime::StackTrace>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "asyncStackTraceId")]
            pub async_stack_trace_id: Option<Runtime::StackTraceId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResumeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SearchInContentReturnObject {
            #[serde(rename = "result")]
            pub result: Vec<SearchMatch>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAsyncCallStackDepthReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBlackboxPatternsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBlackboxedRangesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetBreakpointReturnObject {
            #[serde(rename = "breakpointId")]
            pub breakpoint_id: BreakpointId,
            #[serde(rename = "actualLocation")]
            pub actual_location: Location,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetInstrumentationBreakpointReturnObject {
            #[serde(rename = "breakpointId")]
            pub breakpoint_id: BreakpointId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetBreakpointByUrlReturnObject {
            #[serde(rename = "breakpointId")]
            pub breakpoint_id: BreakpointId,
            #[serde(rename = "locations")]
            pub locations: Vec<Location>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetBreakpointOnFunctionCallReturnObject {
            #[serde(rename = "breakpointId")]
            pub breakpoint_id: BreakpointId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBreakpointsActiveReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPauseOnExceptionsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetReturnValueReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetScriptSourceReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "callFrames")]
            pub call_frames: Option<Vec<CallFrame>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "stackChanged")]
            pub stack_changed: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "asyncStackTrace")]
            pub async_stack_trace: Option<Runtime::StackTrace>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "asyncStackTraceId")]
            pub async_stack_trace_id: Option<Runtime::StackTraceId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "exceptionDetails")]
            pub exception_details: Option<Runtime::ExceptionDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetSkipAllPausesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetVariableValueReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StepIntoReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StepOutReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StepOverReturnObject {}
        impl Method for ContinueToLocation {
            const NAME: &'static str = "Debugger.continueToLocation";
            type ReturnObject = ContinueToLocationReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Debugger.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Debugger.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for EvaluateOnCallFrame {
            const NAME: &'static str = "Debugger.evaluateOnCallFrame";
            type ReturnObject = EvaluateOnCallFrameReturnObject;
        }
        impl Method for GetPossibleBreakpoints {
            const NAME: &'static str = "Debugger.getPossibleBreakpoints";
            type ReturnObject = GetPossibleBreakpointsReturnObject;
        }
        impl Method for GetScriptSource {
            const NAME: &'static str = "Debugger.getScriptSource";
            type ReturnObject = GetScriptSourceReturnObject;
        }
        impl Method for GetWasmBytecode {
            const NAME: &'static str = "Debugger.getWasmBytecode";
            type ReturnObject = GetWasmBytecodeReturnObject;
        }
        impl Method for GetStackTrace {
            const NAME: &'static str = "Debugger.getStackTrace";
            type ReturnObject = GetStackTraceReturnObject;
        }
        impl Method for Pause {
            const NAME: &'static str = "Debugger.pause";
            type ReturnObject = PauseReturnObject;
        }
        impl Method for PauseOnAsyncCall {
            const NAME: &'static str = "Debugger.pauseOnAsyncCall";
            type ReturnObject = PauseOnAsyncCallReturnObject;
        }
        impl Method for RemoveBreakpoint {
            const NAME: &'static str = "Debugger.removeBreakpoint";
            type ReturnObject = RemoveBreakpointReturnObject;
        }
        impl Method for RestartFrame {
            const NAME: &'static str = "Debugger.restartFrame";
            type ReturnObject = RestartFrameReturnObject;
        }
        impl Method for Resume {
            const NAME: &'static str = "Debugger.resume";
            type ReturnObject = ResumeReturnObject;
        }
        impl Method for SearchInContent {
            const NAME: &'static str = "Debugger.searchInContent";
            type ReturnObject = SearchInContentReturnObject;
        }
        impl Method for SetAsyncCallStackDepth {
            const NAME: &'static str = "Debugger.setAsyncCallStackDepth";
            type ReturnObject = SetAsyncCallStackDepthReturnObject;
        }
        impl Method for SetBlackboxPatterns {
            const NAME: &'static str = "Debugger.setBlackboxPatterns";
            type ReturnObject = SetBlackboxPatternsReturnObject;
        }
        impl Method for SetBlackboxedRanges {
            const NAME: &'static str = "Debugger.setBlackboxedRanges";
            type ReturnObject = SetBlackboxedRangesReturnObject;
        }
        impl Method for SetBreakpoint {
            const NAME: &'static str = "Debugger.setBreakpoint";
            type ReturnObject = SetBreakpointReturnObject;
        }
        impl Method for SetInstrumentationBreakpoint {
            const NAME: &'static str = "Debugger.setInstrumentationBreakpoint";
            type ReturnObject = SetInstrumentationBreakpointReturnObject;
        }
        impl Method for SetBreakpointByUrl {
            const NAME: &'static str = "Debugger.setBreakpointByUrl";
            type ReturnObject = SetBreakpointByUrlReturnObject;
        }
        impl Method for SetBreakpointOnFunctionCall {
            const NAME: &'static str = "Debugger.setBreakpointOnFunctionCall";
            type ReturnObject = SetBreakpointOnFunctionCallReturnObject;
        }
        impl Method for SetBreakpointsActive {
            const NAME: &'static str = "Debugger.setBreakpointsActive";
            type ReturnObject = SetBreakpointsActiveReturnObject;
        }
        impl Method for SetPauseOnExceptions {
            const NAME: &'static str = "Debugger.setPauseOnExceptions";
            type ReturnObject = SetPauseOnExceptionsReturnObject;
        }
        impl Method for SetReturnValue {
            const NAME: &'static str = "Debugger.setReturnValue";
            type ReturnObject = SetReturnValueReturnObject;
        }
        impl Method for SetScriptSource {
            const NAME: &'static str = "Debugger.setScriptSource";
            type ReturnObject = SetScriptSourceReturnObject;
        }
        impl Method for SetSkipAllPauses {
            const NAME: &'static str = "Debugger.setSkipAllPauses";
            type ReturnObject = SetSkipAllPausesReturnObject;
        }
        impl Method for SetVariableValue {
            const NAME: &'static str = "Debugger.setVariableValue";
            type ReturnObject = SetVariableValueReturnObject;
        }
        impl Method for StepInto {
            const NAME: &'static str = "Debugger.stepInto";
            type ReturnObject = StepIntoReturnObject;
        }
        impl Method for StepOut {
            const NAME: &'static str = "Debugger.stepOut";
            type ReturnObject = StepOutReturnObject;
        }
        impl Method for StepOver {
            const NAME: &'static str = "Debugger.stepOver";
            type ReturnObject = StepOverReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct BreakpointResolvedEvent {
                pub params: BreakpointResolvedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct BreakpointResolvedEventParams {
                #[serde(rename = "breakpointId")]
                pub breakpoint_id: super::BreakpointId,
                #[serde(rename = "location")]
                pub location: super::Location,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PausedEvent {
                pub params: PausedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PausedEventParams {
                #[serde(rename = "callFrames")]
                pub call_frames: Vec<super::CallFrame>,
                #[serde(rename = "reason")]
                pub reason: super::PausedEventReasonOption,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "hitBreakpoints")]
                pub hit_breakpoints: Option<Vec<String>>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "asyncStackTrace")]
                pub async_stack_trace: Option<super::super::Runtime::StackTrace>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "asyncStackTraceId")]
                pub async_stack_trace_id: Option<super::super::Runtime::StackTraceId>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "asyncCallStackTraceId")]
                pub async_call_stack_trace_id: Option<super::super::Runtime::StackTraceId>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ResumedEvent(pub Option<serde_json::Value>);
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ScriptFailedToParseEvent {
                pub params: ScriptFailedToParseEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ScriptFailedToParseEventParams {
                #[serde(rename = "scriptId")]
                pub script_id: super::super::Runtime::ScriptId,
                #[serde(default)]
                #[serde(rename = "url")]
                pub url: String,
                #[serde(default)]
                #[serde(rename = "startLine")]
                pub start_line: JsUInt,
                #[serde(default)]
                #[serde(rename = "startColumn")]
                pub start_column: JsUInt,
                #[serde(default)]
                #[serde(rename = "endLine")]
                pub end_line: JsUInt,
                #[serde(default)]
                #[serde(rename = "endColumn")]
                pub end_column: JsUInt,
                #[serde(rename = "executionContextId")]
                pub execution_context_id: super::super::Runtime::ExecutionContextId,
                #[serde(default)]
                #[serde(rename = "hash")]
                pub hash: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "sourceMapURL")]
                pub source_map_url: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "hasSourceURL")]
                pub has_source_url: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "isModule")]
                pub is_module: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "length")]
                pub length: Option<JsUInt>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "stackTrace")]
                pub stack_trace: Option<super::super::Runtime::StackTrace>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "codeOffset")]
                pub code_offset: Option<JsUInt>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "scriptLanguage")]
                pub script_language: Option<super::super::Debugger::ScriptLanguage>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "embedderName")]
                pub embedder_name: Option<String>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ScriptParsedEvent {
                pub params: ScriptParsedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ScriptParsedEventParams {
                #[serde(rename = "scriptId")]
                pub script_id: super::super::Runtime::ScriptId,
                #[serde(default)]
                #[serde(rename = "url")]
                pub url: String,
                #[serde(default)]
                #[serde(rename = "startLine")]
                pub start_line: JsUInt,
                #[serde(default)]
                #[serde(rename = "startColumn")]
                pub start_column: JsUInt,
                #[serde(default)]
                #[serde(rename = "endLine")]
                pub end_line: JsUInt,
                #[serde(default)]
                #[serde(rename = "endColumn")]
                pub end_column: JsUInt,
                #[serde(rename = "executionContextId")]
                pub execution_context_id: super::super::Runtime::ExecutionContextId,
                #[serde(default)]
                #[serde(rename = "hash")]
                pub hash: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "isLiveEdit")]
                pub is_live_edit: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "sourceMapURL")]
                pub source_map_url: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "hasSourceURL")]
                pub has_source_url: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "isModule")]
                pub is_module: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "length")]
                pub length: Option<JsUInt>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "stackTrace")]
                pub stack_trace: Option<super::super::Runtime::StackTrace>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "codeOffset")]
                pub code_offset: Option<JsUInt>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "scriptLanguage")]
                pub script_language: Option<super::super::Debugger::ScriptLanguage>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "debugSymbols")]
                pub debug_symbols: Option<super::super::Debugger::DebugSymbols>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "embedderName")]
                pub embedder_name: Option<String>,
            }
        }
    }
    pub mod HeapProfiler {
        use super::types::*;
        use super::Runtime;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type HeapSnapshotObjectId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SamplingHeapProfileNode {
            #[serde(rename = "callFrame")]
            pub call_frame: Runtime::CallFrame,
            #[serde(default)]
            #[serde(rename = "selfSize")]
            pub self_size: JsFloat,
            #[serde(default)]
            #[serde(rename = "id")]
            pub id: JsUInt,
            #[serde(rename = "children")]
            pub children: Vec<SamplingHeapProfileNode>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SamplingHeapProfileSample {
            #[serde(default)]
            #[serde(rename = "size")]
            pub size: JsFloat,
            #[serde(default)]
            #[serde(rename = "nodeId")]
            pub node_id: JsUInt,
            #[serde(default)]
            #[serde(rename = "ordinal")]
            pub ordinal: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SamplingHeapProfile {
            #[serde(rename = "head")]
            pub head: SamplingHeapProfileNode,
            #[serde(rename = "samples")]
            pub samples: Vec<SamplingHeapProfileSample>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AddInspectedHeapObject {
            #[serde(rename = "heapObjectId")]
            pub heap_object_id: HeapSnapshotObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CollectGarbage(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetHeapObjectId {
            #[serde(rename = "objectId")]
            pub object_id: Runtime::RemoteObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetObjectByHeapObjectId {
            #[serde(rename = "objectId")]
            pub object_id: HeapSnapshotObjectId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "objectGroup")]
            pub object_group: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetSamplingProfile(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StartSampling {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "samplingInterval")]
            pub sampling_interval: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StartTrackingHeapObjects {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "trackAllocations")]
            pub track_allocations: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopSampling(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StopTrackingHeapObjects {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "reportProgress")]
            pub report_progress: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "treatGlobalObjectsAsRoots")]
            pub treat_global_objects_as_roots: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "captureNumericValue")]
            pub capture_numeric_value: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TakeHeapSnapshot {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "reportProgress")]
            pub report_progress: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "treatGlobalObjectsAsRoots")]
            pub treat_global_objects_as_roots: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "captureNumericValue")]
            pub capture_numeric_value: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddInspectedHeapObjectReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CollectGarbageReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetHeapObjectIdReturnObject {
            #[serde(rename = "heapSnapshotObjectId")]
            pub heap_snapshot_object_id: HeapSnapshotObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetObjectByHeapObjectIdReturnObject {
            #[serde(rename = "result")]
            pub result: Runtime::RemoteObject,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetSamplingProfileReturnObject {
            #[serde(rename = "profile")]
            pub profile: SamplingHeapProfile,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartSamplingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartTrackingHeapObjectsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StopSamplingReturnObject {
            #[serde(rename = "profile")]
            pub profile: SamplingHeapProfile,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopTrackingHeapObjectsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakeHeapSnapshotReturnObject {}
        impl Method for AddInspectedHeapObject {
            const NAME: &'static str = "HeapProfiler.addInspectedHeapObject";
            type ReturnObject = AddInspectedHeapObjectReturnObject;
        }
        impl Method for CollectGarbage {
            const NAME: &'static str = "HeapProfiler.collectGarbage";
            type ReturnObject = CollectGarbageReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "HeapProfiler.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "HeapProfiler.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetHeapObjectId {
            const NAME: &'static str = "HeapProfiler.getHeapObjectId";
            type ReturnObject = GetHeapObjectIdReturnObject;
        }
        impl Method for GetObjectByHeapObjectId {
            const NAME: &'static str = "HeapProfiler.getObjectByHeapObjectId";
            type ReturnObject = GetObjectByHeapObjectIdReturnObject;
        }
        impl Method for GetSamplingProfile {
            const NAME: &'static str = "HeapProfiler.getSamplingProfile";
            type ReturnObject = GetSamplingProfileReturnObject;
        }
        impl Method for StartSampling {
            const NAME: &'static str = "HeapProfiler.startSampling";
            type ReturnObject = StartSamplingReturnObject;
        }
        impl Method for StartTrackingHeapObjects {
            const NAME: &'static str = "HeapProfiler.startTrackingHeapObjects";
            type ReturnObject = StartTrackingHeapObjectsReturnObject;
        }
        impl Method for StopSampling {
            const NAME: &'static str = "HeapProfiler.stopSampling";
            type ReturnObject = StopSamplingReturnObject;
        }
        impl Method for StopTrackingHeapObjects {
            const NAME: &'static str = "HeapProfiler.stopTrackingHeapObjects";
            type ReturnObject = StopTrackingHeapObjectsReturnObject;
        }
        impl Method for TakeHeapSnapshot {
            const NAME: &'static str = "HeapProfiler.takeHeapSnapshot";
            type ReturnObject = TakeHeapSnapshotReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AddHeapSnapshotChunkEvent {
                pub params: AddHeapSnapshotChunkEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AddHeapSnapshotChunkEventParams {
                #[serde(default)]
                #[serde(rename = "chunk")]
                pub chunk: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct HeapStatsUpdateEvent {
                pub params: HeapStatsUpdateEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct HeapStatsUpdateEventParams {
                #[serde(default)]
                #[serde(rename = "statsUpdate")]
                pub stats_update: Vec<JsUInt>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LastSeenObjectIdEvent {
                pub params: LastSeenObjectIdEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LastSeenObjectIdEventParams {
                #[serde(default)]
                #[serde(rename = "lastSeenObjectId")]
                pub last_seen_object_id: JsUInt,
                #[serde(default)]
                #[serde(rename = "timestamp")]
                pub timestamp: JsFloat,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ReportHeapSnapshotProgressEvent {
                pub params: ReportHeapSnapshotProgressEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ReportHeapSnapshotProgressEventParams {
                #[serde(default)]
                #[serde(rename = "done")]
                pub done: JsUInt,
                #[serde(default)]
                #[serde(rename = "total")]
                pub total: JsUInt,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "finished")]
                pub finished: Option<bool>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ResetProfilesEvent(pub Option<serde_json::Value>);
        }
    }
    pub mod Profiler {
        use super::types::*;
        use super::Debugger;
        use super::Runtime;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ProfileNode {
            #[serde(default)]
            #[serde(rename = "id")]
            pub id: JsUInt,
            #[serde(rename = "callFrame")]
            pub call_frame: Runtime::CallFrame,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "hitCount")]
            pub hit_count: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "children")]
            pub children: Option<Vec<JsUInt>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "deoptReason")]
            pub deopt_reason: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "positionTicks")]
            pub position_ticks: Option<Vec<PositionTickInfo>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Profile {
            #[serde(rename = "nodes")]
            pub nodes: Vec<ProfileNode>,
            #[serde(default)]
            #[serde(rename = "startTime")]
            pub start_time: JsFloat,
            #[serde(default)]
            #[serde(rename = "endTime")]
            pub end_time: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "samples")]
            pub samples: Option<Vec<JsUInt>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "timeDeltas")]
            pub time_deltas: Option<Vec<JsUInt>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PositionTickInfo {
            #[serde(default)]
            #[serde(rename = "line")]
            pub line: JsUInt,
            #[serde(default)]
            #[serde(rename = "ticks")]
            pub ticks: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CoverageRange {
            #[serde(default)]
            #[serde(rename = "startOffset")]
            pub start_offset: JsUInt,
            #[serde(default)]
            #[serde(rename = "endOffset")]
            pub end_offset: JsUInt,
            #[serde(default)]
            #[serde(rename = "count")]
            pub count: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct FunctionCoverage {
            #[serde(default)]
            #[serde(rename = "functionName")]
            pub function_name: String,
            #[serde(rename = "ranges")]
            pub ranges: Vec<CoverageRange>,
            #[serde(default)]
            #[serde(rename = "isBlockCoverage")]
            pub is_block_coverage: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ScriptCoverage {
            #[serde(rename = "scriptId")]
            pub script_id: Runtime::ScriptId,
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(rename = "functions")]
            pub functions: Vec<FunctionCoverage>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TypeObject {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TypeProfileEntry {
            #[serde(default)]
            #[serde(rename = "offset")]
            pub offset: JsUInt,
            #[serde(rename = "types")]
            pub Types: Vec<TypeObject>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ScriptTypeProfile {
            #[serde(rename = "scriptId")]
            pub script_id: Runtime::ScriptId,
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(rename = "entries")]
            pub entries: Vec<TypeProfileEntry>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetBestEffortCoverage(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetSamplingInterval {
            #[serde(default)]
            #[serde(rename = "interval")]
            pub interval: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Start(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StartPreciseCoverage {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "callCount")]
            pub call_count: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "detailed")]
            pub detailed: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "allowTriggeredUpdates")]
            pub allow_triggered_updates: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartTypeProfile(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Stop(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopPreciseCoverage(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopTypeProfile(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakePreciseCoverage(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakeTypeProfile(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetBestEffortCoverageReturnObject {
            #[serde(rename = "result")]
            pub result: Vec<ScriptCoverage>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetSamplingIntervalReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StartPreciseCoverageReturnObject {
            #[serde(default)]
            #[serde(rename = "timestamp")]
            pub timestamp: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartTypeProfileReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StopReturnObject {
            #[serde(rename = "profile")]
            pub profile: Profile,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopPreciseCoverageReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopTypeProfileReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TakePreciseCoverageReturnObject {
            #[serde(rename = "result")]
            pub result: Vec<ScriptCoverage>,
            #[serde(default)]
            #[serde(rename = "timestamp")]
            pub timestamp: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TakeTypeProfileReturnObject {
            #[serde(rename = "result")]
            pub result: Vec<ScriptTypeProfile>,
        }
        impl Method for Disable {
            const NAME: &'static str = "Profiler.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Profiler.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetBestEffortCoverage {
            const NAME: &'static str = "Profiler.getBestEffortCoverage";
            type ReturnObject = GetBestEffortCoverageReturnObject;
        }
        impl Method for SetSamplingInterval {
            const NAME: &'static str = "Profiler.setSamplingInterval";
            type ReturnObject = SetSamplingIntervalReturnObject;
        }
        impl Method for Start {
            const NAME: &'static str = "Profiler.start";
            type ReturnObject = StartReturnObject;
        }
        impl Method for StartPreciseCoverage {
            const NAME: &'static str = "Profiler.startPreciseCoverage";
            type ReturnObject = StartPreciseCoverageReturnObject;
        }
        impl Method for StartTypeProfile {
            const NAME: &'static str = "Profiler.startTypeProfile";
            type ReturnObject = StartTypeProfileReturnObject;
        }
        impl Method for Stop {
            const NAME: &'static str = "Profiler.stop";
            type ReturnObject = StopReturnObject;
        }
        impl Method for StopPreciseCoverage {
            const NAME: &'static str = "Profiler.stopPreciseCoverage";
            type ReturnObject = StopPreciseCoverageReturnObject;
        }
        impl Method for StopTypeProfile {
            const NAME: &'static str = "Profiler.stopTypeProfile";
            type ReturnObject = StopTypeProfileReturnObject;
        }
        impl Method for TakePreciseCoverage {
            const NAME: &'static str = "Profiler.takePreciseCoverage";
            type ReturnObject = TakePreciseCoverageReturnObject;
        }
        impl Method for TakeTypeProfile {
            const NAME: &'static str = "Profiler.takeTypeProfile";
            type ReturnObject = TakeTypeProfileReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ConsoleProfileFinishedEvent {
                pub params: ConsoleProfileFinishedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ConsoleProfileFinishedEventParams {
                #[serde(default)]
                #[serde(rename = "id")]
                pub id: String,
                #[serde(rename = "location")]
                pub location: super::super::Debugger::Location,
                #[serde(rename = "profile")]
                pub profile: super::Profile,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "title")]
                pub title: Option<String>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ConsoleProfileStartedEvent {
                pub params: ConsoleProfileStartedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ConsoleProfileStartedEventParams {
                #[serde(default)]
                #[serde(rename = "id")]
                pub id: String,
                #[serde(rename = "location")]
                pub location: super::super::Debugger::Location,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "title")]
                pub title: Option<String>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PreciseCoverageDeltaUpdateEvent {
                pub params: PreciseCoverageDeltaUpdateEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PreciseCoverageDeltaUpdateEventParams {
                #[serde(default)]
                #[serde(rename = "timestamp")]
                pub timestamp: JsFloat,
                #[serde(default)]
                #[serde(rename = "occasion")]
                pub occasion: String,
                #[serde(rename = "result")]
                pub result: Vec<super::ScriptCoverage>,
            }
        }
    }
    pub mod Runtime {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type ScriptId = String;
        pub type RemoteObjectId = String;
        pub type UnserializableValue = String;
        pub type ExecutionContextId = JsUInt;
        pub type Timestamp = JsFloat;
        pub type TimeDelta = JsFloat;
        pub type UniqueDebuggerId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum RemoteObjectType {
            #[serde(rename = "object")]
            Object,
            #[serde(rename = "function")]
            Function,
            #[serde(rename = "undefined")]
            Undefined,
            #[serde(rename = "string")]
            String,
            #[serde(rename = "number")]
            Number,
            #[serde(rename = "boolean")]
            Boolean,
            #[serde(rename = "symbol")]
            Symbol,
            #[serde(rename = "bigint")]
            Bigint,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum RemoteObjectSubtype {
            #[serde(rename = "array")]
            Array,
            #[serde(rename = "null")]
            Null,
            #[serde(rename = "node")]
            Node,
            #[serde(rename = "regexp")]
            Regexp,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "map")]
            Map,
            #[serde(rename = "set")]
            Set,
            #[serde(rename = "weakmap")]
            Weakmap,
            #[serde(rename = "weakset")]
            Weakset,
            #[serde(rename = "iterator")]
            Iterator,
            #[serde(rename = "generator")]
            Generator,
            #[serde(rename = "error")]
            Error,
            #[serde(rename = "proxy")]
            Proxy,
            #[serde(rename = "promise")]
            Promise,
            #[serde(rename = "typedarray")]
            Typedarray,
            #[serde(rename = "arraybuffer")]
            Arraybuffer,
            #[serde(rename = "dataview")]
            Dataview,
            #[serde(rename = "webassemblymemory")]
            Webassemblymemory,
            #[serde(rename = "wasmvalue")]
            Wasmvalue,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ObjectPreviewType {
            #[serde(rename = "object")]
            Object,
            #[serde(rename = "function")]
            Function,
            #[serde(rename = "undefined")]
            Undefined,
            #[serde(rename = "string")]
            String,
            #[serde(rename = "number")]
            Number,
            #[serde(rename = "boolean")]
            Boolean,
            #[serde(rename = "symbol")]
            Symbol,
            #[serde(rename = "bigint")]
            Bigint,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ObjectPreviewSubtype {
            #[serde(rename = "array")]
            Array,
            #[serde(rename = "null")]
            Null,
            #[serde(rename = "node")]
            Node,
            #[serde(rename = "regexp")]
            Regexp,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "map")]
            Map,
            #[serde(rename = "set")]
            Set,
            #[serde(rename = "weakmap")]
            Weakmap,
            #[serde(rename = "weakset")]
            Weakset,
            #[serde(rename = "iterator")]
            Iterator,
            #[serde(rename = "generator")]
            Generator,
            #[serde(rename = "error")]
            Error,
            #[serde(rename = "proxy")]
            Proxy,
            #[serde(rename = "promise")]
            Promise,
            #[serde(rename = "typedarray")]
            Typedarray,
            #[serde(rename = "arraybuffer")]
            Arraybuffer,
            #[serde(rename = "dataview")]
            Dataview,
            #[serde(rename = "webassemblymemory")]
            Webassemblymemory,
            #[serde(rename = "wasmvalue")]
            Wasmvalue,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum PropertyPreviewType {
            #[serde(rename = "object")]
            Object,
            #[serde(rename = "function")]
            Function,
            #[serde(rename = "undefined")]
            Undefined,
            #[serde(rename = "string")]
            String,
            #[serde(rename = "number")]
            Number,
            #[serde(rename = "boolean")]
            Boolean,
            #[serde(rename = "symbol")]
            Symbol,
            #[serde(rename = "accessor")]
            Accessor,
            #[serde(rename = "bigint")]
            Bigint,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum PropertyPreviewSubtype {
            #[serde(rename = "array")]
            Array,
            #[serde(rename = "null")]
            Null,
            #[serde(rename = "node")]
            Node,
            #[serde(rename = "regexp")]
            Regexp,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "map")]
            Map,
            #[serde(rename = "set")]
            Set,
            #[serde(rename = "weakmap")]
            Weakmap,
            #[serde(rename = "weakset")]
            Weakset,
            #[serde(rename = "iterator")]
            Iterator,
            #[serde(rename = "generator")]
            Generator,
            #[serde(rename = "error")]
            Error,
            #[serde(rename = "proxy")]
            Proxy,
            #[serde(rename = "promise")]
            Promise,
            #[serde(rename = "typedarray")]
            Typedarray,
            #[serde(rename = "arraybuffer")]
            Arraybuffer,
            #[serde(rename = "dataview")]
            Dataview,
            #[serde(rename = "webassemblymemory")]
            Webassemblymemory,
            #[serde(rename = "wasmvalue")]
            Wasmvalue,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ConsoleAPICalledEventTypeOption {
            #[serde(rename = "log")]
            Log,
            #[serde(rename = "debug")]
            Debug,
            #[serde(rename = "info")]
            Info,
            #[serde(rename = "error")]
            Error,
            #[serde(rename = "warning")]
            Warning,
            #[serde(rename = "dir")]
            Dir,
            #[serde(rename = "dirxml")]
            Dirxml,
            #[serde(rename = "table")]
            Table,
            #[serde(rename = "trace")]
            Trace,
            #[serde(rename = "clear")]
            Clear,
            #[serde(rename = "startGroup")]
            StartGroup,
            #[serde(rename = "startGroupCollapsed")]
            StartGroupCollapsed,
            #[serde(rename = "endGroup")]
            EndGroup,
            #[serde(rename = "assert")]
            Assert,
            #[serde(rename = "profile")]
            Profile,
            #[serde(rename = "profileEnd")]
            ProfileEnd,
            #[serde(rename = "count")]
            Count,
            #[serde(rename = "timeEnd")]
            TimeEnd,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RemoteObject {
            #[serde(rename = "type")]
            pub Type: RemoteObjectType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "subtype")]
            pub subtype: Option<RemoteObjectSubtype>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "className")]
            pub class_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: Option<Json>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "unserializableValue")]
            pub unserializable_value: Option<UnserializableValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "description")]
            pub description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "objectId")]
            pub object_id: Option<RemoteObjectId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "preview")]
            pub preview: Option<ObjectPreview>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "customPreview")]
            pub custom_preview: Option<CustomPreview>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CustomPreview {
            #[serde(default)]
            #[serde(rename = "header")]
            pub header: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "bodyGetterId")]
            pub body_getter_id: Option<RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ObjectPreview {
            #[serde(rename = "type")]
            pub Type: ObjectPreviewType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "subtype")]
            pub subtype: Option<ObjectPreviewSubtype>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "description")]
            pub description: Option<String>,
            #[serde(default)]
            #[serde(rename = "overflow")]
            pub overflow: bool,
            #[serde(rename = "properties")]
            pub properties: Vec<PropertyPreview>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "entries")]
            pub entries: Option<Vec<EntryPreview>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PropertyPreview {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(rename = "type")]
            pub Type: PropertyPreviewType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "valuePreview")]
            pub value_preview: Option<ObjectPreview>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "subtype")]
            pub subtype: Option<PropertyPreviewSubtype>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct EntryPreview {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "key")]
            pub key: Option<ObjectPreview>,
            #[serde(rename = "value")]
            pub value: ObjectPreview,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PropertyDescriptor {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "value")]
            pub value: Option<RemoteObject>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "writable")]
            pub writable: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "get")]
            pub get: Option<RemoteObject>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "set")]
            pub set: Option<RemoteObject>,
            #[serde(default)]
            #[serde(rename = "configurable")]
            pub configurable: bool,
            #[serde(default)]
            #[serde(rename = "enumerable")]
            pub enumerable: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "wasThrown")]
            pub was_thrown: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "isOwn")]
            pub is_own: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "symbol")]
            pub symbol: Option<RemoteObject>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct InternalPropertyDescriptor {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "value")]
            pub value: Option<RemoteObject>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PrivatePropertyDescriptor {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "value")]
            pub value: Option<RemoteObject>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "get")]
            pub get: Option<RemoteObject>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "set")]
            pub set: Option<RemoteObject>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CallArgument {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: Option<Json>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "unserializableValue")]
            pub unserializable_value: Option<UnserializableValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "objectId")]
            pub object_id: Option<RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ExecutionContextDescription {
            #[serde(rename = "id")]
            pub id: ExecutionContextId,
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: String,
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "uniqueId")]
            pub unique_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ExceptionDetails {
            #[serde(default)]
            #[serde(rename = "exceptionId")]
            pub exception_id: JsUInt,
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: String,
            #[serde(default)]
            #[serde(rename = "lineNumber")]
            pub line_number: JsUInt,
            #[serde(default)]
            #[serde(rename = "columnNumber")]
            pub column_number: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "scriptId")]
            pub script_id: Option<ScriptId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "stackTrace")]
            pub stack_trace: Option<StackTrace>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "exception")]
            pub exception: Option<RemoteObject>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "executionContextId")]
            pub execution_context_id: Option<ExecutionContextId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CallFrame {
            #[serde(default)]
            #[serde(rename = "functionName")]
            pub function_name: String,
            #[serde(rename = "scriptId")]
            pub script_id: ScriptId,
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(default)]
            #[serde(rename = "lineNumber")]
            pub line_number: JsUInt,
            #[serde(default)]
            #[serde(rename = "columnNumber")]
            pub column_number: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StackTrace {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "description")]
            pub description: Option<String>,
            #[serde(rename = "callFrames")]
            pub call_frames: Vec<CallFrame>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "parent")]
            pub parent: Option<Box<StackTrace>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "parentId")]
            pub parent_id: Option<StackTraceId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StackTraceId {
            #[serde(default)]
            #[serde(rename = "id")]
            pub id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "debuggerId")]
            pub debugger_id: Option<UniqueDebuggerId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AwaitPromise {
            #[serde(rename = "promiseObjectId")]
            pub promise_object_id: RemoteObjectId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "returnByValue")]
            pub return_by_value: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "generatePreview")]
            pub generate_preview: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CallFunctionOn {
            #[serde(default)]
            #[serde(rename = "functionDeclaration")]
            pub function_declaration: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "objectId")]
            pub object_id: Option<RemoteObjectId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "arguments")]
            pub arguments: Option<Vec<CallArgument>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "silent")]
            pub silent: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "returnByValue")]
            pub return_by_value: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "generatePreview")]
            pub generate_preview: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "userGesture")]
            pub user_gesture: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "awaitPromise")]
            pub await_promise: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "executionContextId")]
            pub execution_context_id: Option<ExecutionContextId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "objectGroup")]
            pub object_group: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "throwOnSideEffect")]
            pub throw_on_side_effect: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CompileScript {
            #[serde(default)]
            #[serde(rename = "expression")]
            pub expression: String,
            #[serde(default)]
            #[serde(rename = "sourceURL")]
            pub source_url: String,
            #[serde(default)]
            #[serde(rename = "persistScript")]
            pub persist_script: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "executionContextId")]
            pub execution_context_id: Option<ExecutionContextId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DiscardConsoleEntries(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Evaluate {
            #[serde(default)]
            #[serde(rename = "expression")]
            pub expression: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "objectGroup")]
            pub object_group: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "includeCommandLineAPI")]
            pub include_command_line_api: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "silent")]
            pub silent: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "contextId")]
            pub context_id: Option<ExecutionContextId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "returnByValue")]
            pub return_by_value: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "generatePreview")]
            pub generate_preview: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "userGesture")]
            pub user_gesture: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "awaitPromise")]
            pub await_promise: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "throwOnSideEffect")]
            pub throw_on_side_effect: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "timeout")]
            pub timeout: Option<TimeDelta>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "disableBreaks")]
            pub disable_breaks: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "replMode")]
            pub repl_mode: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "allowUnsafeEvalBlockedByCSP")]
            pub allow_unsafe_eval_blocked_by_csp: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "uniqueContextId")]
            pub unique_context_id: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetIsolateId(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetHeapUsage(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetProperties {
            #[serde(rename = "objectId")]
            pub object_id: RemoteObjectId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "ownProperties")]
            pub own_properties: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "accessorPropertiesOnly")]
            pub accessor_properties_only: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "generatePreview")]
            pub generate_preview: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "nonIndexedPropertiesOnly")]
            pub non_indexed_properties_only: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GlobalLexicalScopeNames {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "executionContextId")]
            pub execution_context_id: Option<ExecutionContextId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct QueryObjects {
            #[serde(rename = "prototypeObjectId")]
            pub protoType_object_id: RemoteObjectId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "objectGroup")]
            pub object_group: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ReleaseObject {
            #[serde(rename = "objectId")]
            pub object_id: RemoteObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ReleaseObjectGroup {
            #[serde(default)]
            #[serde(rename = "objectGroup")]
            pub object_group: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RunIfWaitingForDebugger(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RunScript {
            #[serde(rename = "scriptId")]
            pub script_id: ScriptId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "executionContextId")]
            pub execution_context_id: Option<ExecutionContextId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "objectGroup")]
            pub object_group: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "silent")]
            pub silent: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "includeCommandLineAPI")]
            pub include_command_line_api: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "returnByValue")]
            pub return_by_value: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "generatePreview")]
            pub generate_preview: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "awaitPromise")]
            pub await_promise: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetAsyncCallStackDepth {
            #[serde(default)]
            #[serde(rename = "maxDepth")]
            pub max_depth: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetCustomObjectFormatterEnabled {
            #[serde(default)]
            #[serde(rename = "enabled")]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetMaxCallStackSizeToCapture {
            #[serde(default)]
            #[serde(rename = "size")]
            pub size: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TerminateExecution(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AddBinding {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "executionContextId")]
            pub execution_context_id: Option<ExecutionContextId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "executionContextName")]
            pub execution_context_name: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RemoveBinding {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AwaitPromiseReturnObject {
            #[serde(rename = "result")]
            pub result: RemoteObject,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "exceptionDetails")]
            pub exception_details: Option<ExceptionDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CallFunctionOnReturnObject {
            #[serde(rename = "result")]
            pub result: RemoteObject,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "exceptionDetails")]
            pub exception_details: Option<ExceptionDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CompileScriptReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "scriptId")]
            pub script_id: Option<ScriptId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "exceptionDetails")]
            pub exception_details: Option<ExceptionDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DiscardConsoleEntriesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct EvaluateReturnObject {
            #[serde(rename = "result")]
            pub result: RemoteObject,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "exceptionDetails")]
            pub exception_details: Option<ExceptionDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetIsolateIdReturnObject {
            #[serde(default)]
            #[serde(rename = "id")]
            pub id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetHeapUsageReturnObject {
            #[serde(default)]
            #[serde(rename = "usedSize")]
            pub used_size: JsFloat,
            #[serde(default)]
            #[serde(rename = "totalSize")]
            pub total_size: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetPropertiesReturnObject {
            #[serde(rename = "result")]
            pub result: Vec<PropertyDescriptor>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "internalProperties")]
            pub internal_properties: Option<Vec<InternalPropertyDescriptor>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "privateProperties")]
            pub private_properties: Option<Vec<PrivatePropertyDescriptor>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "exceptionDetails")]
            pub exception_details: Option<ExceptionDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GlobalLexicalScopeNamesReturnObject {
            #[serde(rename = "names")]
            pub names: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct QueryObjectsReturnObject {
            #[serde(rename = "objects")]
            pub objects: RemoteObject,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReleaseObjectReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReleaseObjectGroupReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RunIfWaitingForDebuggerReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RunScriptReturnObject {
            #[serde(rename = "result")]
            pub result: RemoteObject,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "exceptionDetails")]
            pub exception_details: Option<ExceptionDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAsyncCallStackDepthReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetCustomObjectFormatterEnabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetMaxCallStackSizeToCaptureReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TerminateExecutionReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddBindingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveBindingReturnObject {}
        impl Method for AwaitPromise {
            const NAME: &'static str = "Runtime.awaitPromise";
            type ReturnObject = AwaitPromiseReturnObject;
        }
        impl Method for CallFunctionOn {
            const NAME: &'static str = "Runtime.callFunctionOn";
            type ReturnObject = CallFunctionOnReturnObject;
        }
        impl Method for CompileScript {
            const NAME: &'static str = "Runtime.compileScript";
            type ReturnObject = CompileScriptReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Runtime.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for DiscardConsoleEntries {
            const NAME: &'static str = "Runtime.discardConsoleEntries";
            type ReturnObject = DiscardConsoleEntriesReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Runtime.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for Evaluate {
            const NAME: &'static str = "Runtime.evaluate";
            type ReturnObject = EvaluateReturnObject;
        }
        impl Method for GetIsolateId {
            const NAME: &'static str = "Runtime.getIsolateId";
            type ReturnObject = GetIsolateIdReturnObject;
        }
        impl Method for GetHeapUsage {
            const NAME: &'static str = "Runtime.getHeapUsage";
            type ReturnObject = GetHeapUsageReturnObject;
        }
        impl Method for GetProperties {
            const NAME: &'static str = "Runtime.getProperties";
            type ReturnObject = GetPropertiesReturnObject;
        }
        impl Method for GlobalLexicalScopeNames {
            const NAME: &'static str = "Runtime.globalLexicalScopeNames";
            type ReturnObject = GlobalLexicalScopeNamesReturnObject;
        }
        impl Method for QueryObjects {
            const NAME: &'static str = "Runtime.queryObjects";
            type ReturnObject = QueryObjectsReturnObject;
        }
        impl Method for ReleaseObject {
            const NAME: &'static str = "Runtime.releaseObject";
            type ReturnObject = ReleaseObjectReturnObject;
        }
        impl Method for ReleaseObjectGroup {
            const NAME: &'static str = "Runtime.releaseObjectGroup";
            type ReturnObject = ReleaseObjectGroupReturnObject;
        }
        impl Method for RunIfWaitingForDebugger {
            const NAME: &'static str = "Runtime.runIfWaitingForDebugger";
            type ReturnObject = RunIfWaitingForDebuggerReturnObject;
        }
        impl Method for RunScript {
            const NAME: &'static str = "Runtime.runScript";
            type ReturnObject = RunScriptReturnObject;
        }
        impl Method for SetAsyncCallStackDepth {
            const NAME: &'static str = "Runtime.setAsyncCallStackDepth";
            type ReturnObject = SetAsyncCallStackDepthReturnObject;
        }
        impl Method for SetCustomObjectFormatterEnabled {
            const NAME: &'static str = "Runtime.setCustomObjectFormatterEnabled";
            type ReturnObject = SetCustomObjectFormatterEnabledReturnObject;
        }
        impl Method for SetMaxCallStackSizeToCapture {
            const NAME: &'static str = "Runtime.setMaxCallStackSizeToCapture";
            type ReturnObject = SetMaxCallStackSizeToCaptureReturnObject;
        }
        impl Method for TerminateExecution {
            const NAME: &'static str = "Runtime.terminateExecution";
            type ReturnObject = TerminateExecutionReturnObject;
        }
        impl Method for AddBinding {
            const NAME: &'static str = "Runtime.addBinding";
            type ReturnObject = AddBindingReturnObject;
        }
        impl Method for RemoveBinding {
            const NAME: &'static str = "Runtime.removeBinding";
            type ReturnObject = RemoveBindingReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct BindingCalledEvent {
                pub params: BindingCalledEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct BindingCalledEventParams {
                #[serde(default)]
                #[serde(rename = "name")]
                pub name: String,
                #[serde(default)]
                #[serde(rename = "payload")]
                pub payload: String,
                #[serde(rename = "executionContextId")]
                pub execution_context_id: super::ExecutionContextId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ConsoleAPICalledEvent {
                pub params: ConsoleAPICalledEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ConsoleAPICalledEventParams {
                #[serde(rename = "type")]
                pub Type: super::ConsoleAPICalledEventTypeOption,
                #[serde(rename = "args")]
                pub args: Vec<super::RemoteObject>,
                #[serde(rename = "executionContextId")]
                pub execution_context_id: super::ExecutionContextId,
                #[serde(rename = "timestamp")]
                pub timestamp: super::Timestamp,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "stackTrace")]
                pub stack_trace: Option<super::StackTrace>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "context")]
                pub context: Option<String>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ExceptionRevokedEvent {
                pub params: ExceptionRevokedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ExceptionRevokedEventParams {
                #[serde(default)]
                #[serde(rename = "reason")]
                pub reason: String,
                #[serde(default)]
                #[serde(rename = "exceptionId")]
                pub exception_id: JsUInt,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ExceptionThrownEvent {
                pub params: ExceptionThrownEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ExceptionThrownEventParams {
                #[serde(rename = "timestamp")]
                pub timestamp: super::Timestamp,
                #[serde(rename = "exceptionDetails")]
                pub exception_details: super::ExceptionDetails,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ExecutionContextCreatedEvent {
                pub params: ExecutionContextCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ExecutionContextCreatedEventParams {
                #[serde(rename = "context")]
                pub context: super::ExecutionContextDescription,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ExecutionContextDestroyedEvent {
                pub params: ExecutionContextDestroyedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ExecutionContextDestroyedEventParams {
                #[serde(rename = "executionContextId")]
                pub execution_context_id: super::ExecutionContextId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ExecutionContextsClearedEvent(pub Option<serde_json::Value>);
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct InspectRequestedEvent {
                pub params: InspectRequestedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct InspectRequestedEventParams {
                #[serde(rename = "object")]
                pub object: super::RemoteObject,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "executionContextId")]
                pub execution_context_id: Option<super::ExecutionContextId>,
            }
        }
    }
    pub mod Schema {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Domain {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "version")]
            pub version: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetDomains(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetDomainsReturnObject {
            #[serde(rename = "domains")]
            pub domains: Vec<Domain>,
        }
        impl Method for GetDomains {
            const NAME: &'static str = "Schema.getDomains";
            type ReturnObject = GetDomainsReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod Accessibility {
        use super::types::*;
        use super::Page;
        use super::Runtime;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type AXNodeId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum AXValueType {
            #[serde(rename = "boolean")]
            Boolean,
            #[serde(rename = "tristate")]
            Tristate,
            #[serde(rename = "booleanOrUndefined")]
            BooleanOrUndefined,
            #[serde(rename = "idref")]
            Idref,
            #[serde(rename = "idrefList")]
            IdrefList,
            #[serde(rename = "integer")]
            Integer,
            #[serde(rename = "node")]
            Node,
            #[serde(rename = "nodeList")]
            NodeList,
            #[serde(rename = "number")]
            Number,
            #[serde(rename = "string")]
            String,
            #[serde(rename = "computedString")]
            ComputedString,
            #[serde(rename = "token")]
            Token,
            #[serde(rename = "tokenList")]
            TokenList,
            #[serde(rename = "domRelation")]
            DomRelation,
            #[serde(rename = "role")]
            Role,
            #[serde(rename = "internalRole")]
            InternalRole,
            #[serde(rename = "valueUndefined")]
            ValueUndefined,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum AXValueSourceType {
            #[serde(rename = "attribute")]
            Attribute,
            #[serde(rename = "implicit")]
            Implicit,
            #[serde(rename = "style")]
            Style,
            #[serde(rename = "contents")]
            Contents,
            #[serde(rename = "placeholder")]
            Placeholder,
            #[serde(rename = "relatedElement")]
            RelatedElement,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum AXValueNativeSourceType {
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "figcaption")]
            Figcaption,
            #[serde(rename = "label")]
            Label,
            #[serde(rename = "labelfor")]
            Labelfor,
            #[serde(rename = "labelwrapped")]
            Labelwrapped,
            #[serde(rename = "legend")]
            Legend,
            #[serde(rename = "rubyannotation")]
            Rubyannotation,
            #[serde(rename = "tablecaption")]
            Tablecaption,
            #[serde(rename = "title")]
            Title,
            #[serde(rename = "other")]
            Other,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum AXPropertyName {
            #[serde(rename = "busy")]
            Busy,
            #[serde(rename = "disabled")]
            Disabled,
            #[serde(rename = "editable")]
            Editable,
            #[serde(rename = "focusable")]
            Focusable,
            #[serde(rename = "focused")]
            Focused,
            #[serde(rename = "hidden")]
            Hidden,
            #[serde(rename = "hiddenRoot")]
            HiddenRoot,
            #[serde(rename = "invalid")]
            Invalid,
            #[serde(rename = "keyshortcuts")]
            Keyshortcuts,
            #[serde(rename = "settable")]
            Settable,
            #[serde(rename = "roledescription")]
            Roledescription,
            #[serde(rename = "live")]
            Live,
            #[serde(rename = "atomic")]
            Atomic,
            #[serde(rename = "relevant")]
            Relevant,
            #[serde(rename = "root")]
            Root,
            #[serde(rename = "autocomplete")]
            Autocomplete,
            #[serde(rename = "hasPopup")]
            HasPopup,
            #[serde(rename = "level")]
            Level,
            #[serde(rename = "multiselectable")]
            Multiselectable,
            #[serde(rename = "orientation")]
            Orientation,
            #[serde(rename = "multiline")]
            Multiline,
            #[serde(rename = "readonly")]
            Readonly,
            #[serde(rename = "required")]
            Required,
            #[serde(rename = "valuemin")]
            Valuemin,
            #[serde(rename = "valuemax")]
            Valuemax,
            #[serde(rename = "valuetext")]
            Valuetext,
            #[serde(rename = "checked")]
            Checked,
            #[serde(rename = "expanded")]
            Expanded,
            #[serde(rename = "modal")]
            Modal,
            #[serde(rename = "pressed")]
            Pressed,
            #[serde(rename = "selected")]
            Selected,
            #[serde(rename = "activedescendant")]
            Activedescendant,
            #[serde(rename = "controls")]
            Controls,
            #[serde(rename = "describedby")]
            Describedby,
            #[serde(rename = "details")]
            Details,
            #[serde(rename = "errormessage")]
            Errormessage,
            #[serde(rename = "flowto")]
            Flowto,
            #[serde(rename = "labelledby")]
            Labelledby,
            #[serde(rename = "owns")]
            Owns,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AXValueSource {
            #[serde(rename = "type")]
            pub Type: AXValueSourceType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "value")]
            pub value: Option<AXValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "attribute")]
            pub attribute: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "attributeValue")]
            pub attribute_value: Option<AXValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "superseded")]
            pub superseded: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nativeSource")]
            pub native_source: Option<AXValueNativeSourceType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nativeSourceValue")]
            pub native_source_value: Option<AXValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "invalid")]
            pub invalid: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "invalidReason")]
            pub invalid_reason: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AXRelatedNode {
            #[serde(rename = "backendDOMNodeId")]
            pub backend_dom_node_id: DOM::BackendNodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "idref")]
            pub idref: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AXProperty {
            #[serde(rename = "name")]
            pub name: AXPropertyName,
            #[serde(rename = "value")]
            pub value: AXValue,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AXValue {
            #[serde(rename = "type")]
            pub Type: AXValueType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: Option<Json>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "relatedNodes")]
            pub related_nodes: Option<Vec<AXRelatedNode>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "sources")]
            pub sources: Option<Vec<AXValueSource>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AXNode {
            #[serde(rename = "nodeId")]
            pub node_id: AXNodeId,
            #[serde(default)]
            #[serde(rename = "ignored")]
            pub ignored: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "ignoredReasons")]
            pub ignored_reasons: Option<Vec<AXProperty>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "role")]
            pub role: Option<AXValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "name")]
            pub name: Option<AXValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "description")]
            pub description: Option<AXValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "value")]
            pub value: Option<AXValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "properties")]
            pub properties: Option<Vec<AXProperty>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "parentId")]
            pub parent_id: Option<AXNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "childIds")]
            pub child_ids: Option<Vec<AXNodeId>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendDOMNodeId")]
            pub backend_dom_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "frameId")]
            pub frame_id: Option<Page::FrameId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetPartialAXTree {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<DOM::NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "objectId")]
            pub object_id: Option<Runtime::RemoteObjectId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "fetchRelatives")]
            pub fetch_relatives: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetFullAXTree {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "depth")]
            pub depth: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "max_depth")]
            pub max_depth: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "frameId")]
            pub frame_id: Option<Page::FrameId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetRootAXNode {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "frameId")]
            pub frame_id: Option<Page::FrameId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetAXNodeAndAncestors {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<DOM::NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "objectId")]
            pub object_id: Option<Runtime::RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetChildAXNodes {
            #[serde(rename = "id")]
            pub id: AXNodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "frameId")]
            pub frame_id: Option<Page::FrameId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct QueryAXTree {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<DOM::NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "objectId")]
            pub object_id: Option<Runtime::RemoteObjectId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "accessibleName")]
            pub accessible_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "role")]
            pub role: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetPartialAXTreeReturnObject {
            #[serde(rename = "nodes")]
            pub nodes: Vec<AXNode>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetFullAXTreeReturnObject {
            #[serde(rename = "nodes")]
            pub nodes: Vec<AXNode>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetRootAXNodeReturnObject {
            #[serde(rename = "node")]
            pub node: AXNode,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetAXNodeAndAncestorsReturnObject {
            #[serde(rename = "nodes")]
            pub nodes: Vec<AXNode>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetChildAXNodesReturnObject {
            #[serde(rename = "nodes")]
            pub nodes: Vec<AXNode>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct QueryAXTreeReturnObject {
            #[serde(rename = "nodes")]
            pub nodes: Vec<AXNode>,
        }
        impl Method for Disable {
            const NAME: &'static str = "Accessibility.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Accessibility.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetPartialAXTree {
            const NAME: &'static str = "Accessibility.getPartialAXTree";
            type ReturnObject = GetPartialAXTreeReturnObject;
        }
        impl Method for GetFullAXTree {
            const NAME: &'static str = "Accessibility.getFullAXTree";
            type ReturnObject = GetFullAXTreeReturnObject;
        }
        impl Method for GetRootAXNode {
            const NAME: &'static str = "Accessibility.getRootAXNode";
            type ReturnObject = GetRootAXNodeReturnObject;
        }
        impl Method for GetAXNodeAndAncestors {
            const NAME: &'static str = "Accessibility.getAXNodeAndAncestors";
            type ReturnObject = GetAXNodeAndAncestorsReturnObject;
        }
        impl Method for GetChildAXNodes {
            const NAME: &'static str = "Accessibility.getChildAXNodes";
            type ReturnObject = GetChildAXNodesReturnObject;
        }
        impl Method for QueryAXTree {
            const NAME: &'static str = "Accessibility.queryAXTree";
            type ReturnObject = QueryAXTreeReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LoadCompleteEvent {
                pub params: LoadCompleteEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LoadCompleteEventParams {
                #[serde(rename = "root")]
                pub root: super::AXNode,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodesUpdatedEvent {
                pub params: NodesUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodesUpdatedEventParams {
                #[serde(rename = "nodes")]
                pub nodes: Vec<super::AXNode>,
            }
        }
    }
    pub mod Animation {
        use super::types::*;
        use super::Runtime;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum AnimationType {
            #[serde(rename = "CSSTransition")]
            CssTransition,
            #[serde(rename = "CSSAnimation")]
            CssAnimation,
            #[serde(rename = "WebAnimation")]
            WebAnimation,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Animation {
            #[serde(default)]
            #[serde(rename = "id")]
            pub id: String,
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "pausedState")]
            pub paused_state: bool,
            #[serde(default)]
            #[serde(rename = "playState")]
            pub play_state: String,
            #[serde(default)]
            #[serde(rename = "playbackRate")]
            pub playback_rate: JsFloat,
            #[serde(default)]
            #[serde(rename = "startTime")]
            pub start_time: JsFloat,
            #[serde(default)]
            #[serde(rename = "currentTime")]
            pub current_time: JsFloat,
            #[serde(rename = "type")]
            pub Type: AnimationType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "source")]
            pub source: Option<AnimationEffect>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "cssId")]
            pub css_id: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AnimationEffect {
            #[serde(default)]
            #[serde(rename = "delay")]
            pub delay: JsFloat,
            #[serde(default)]
            #[serde(rename = "endDelay")]
            pub end_delay: JsFloat,
            #[serde(default)]
            #[serde(rename = "iterationStart")]
            pub iteration_start: JsFloat,
            #[serde(default)]
            #[serde(rename = "iterations")]
            pub iterations: JsFloat,
            #[serde(default)]
            #[serde(rename = "duration")]
            pub duration: JsFloat,
            #[serde(default)]
            #[serde(rename = "direction")]
            pub direction: String,
            #[serde(default)]
            #[serde(rename = "fill")]
            pub fill: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "keyframesRule")]
            pub keyframes_rule: Option<KeyframesRule>,
            #[serde(default)]
            #[serde(rename = "easing")]
            pub easing: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct KeyframesRule {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: Option<String>,
            #[serde(rename = "keyframes")]
            pub keyframes: Vec<KeyframeStyle>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct KeyframeStyle {
            #[serde(default)]
            #[serde(rename = "offset")]
            pub offset: String,
            #[serde(default)]
            #[serde(rename = "easing")]
            pub easing: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetCurrentTime {
            #[serde(default)]
            #[serde(rename = "id")]
            pub id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetPlaybackRate(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ReleaseAnimations {
            #[serde(default)]
            #[serde(rename = "animations")]
            pub animations: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ResolveAnimation {
            #[serde(default)]
            #[serde(rename = "animationId")]
            pub animation_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SeekAnimations {
            #[serde(default)]
            #[serde(rename = "animations")]
            pub animations: Vec<String>,
            #[serde(default)]
            #[serde(rename = "currentTime")]
            pub current_time: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetPaused {
            #[serde(default)]
            #[serde(rename = "animations")]
            pub animations: Vec<String>,
            #[serde(default)]
            #[serde(rename = "paused")]
            pub paused: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetPlaybackRate {
            #[serde(default)]
            #[serde(rename = "playbackRate")]
            pub playback_rate: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetTiming {
            #[serde(default)]
            #[serde(rename = "animationId")]
            pub animation_id: String,
            #[serde(default)]
            #[serde(rename = "duration")]
            pub duration: JsFloat,
            #[serde(default)]
            #[serde(rename = "delay")]
            pub delay: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetCurrentTimeReturnObject {
            #[serde(default)]
            #[serde(rename = "currentTime")]
            pub current_time: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetPlaybackRateReturnObject {
            #[serde(default)]
            #[serde(rename = "playbackRate")]
            pub playback_rate: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReleaseAnimationsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ResolveAnimationReturnObject {
            #[serde(rename = "remoteObject")]
            pub remote_object: Runtime::RemoteObject,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SeekAnimationsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPausedReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPlaybackRateReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetTimingReturnObject {}
        impl Method for Disable {
            const NAME: &'static str = "Animation.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Animation.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetCurrentTime {
            const NAME: &'static str = "Animation.getCurrentTime";
            type ReturnObject = GetCurrentTimeReturnObject;
        }
        impl Method for GetPlaybackRate {
            const NAME: &'static str = "Animation.getPlaybackRate";
            type ReturnObject = GetPlaybackRateReturnObject;
        }
        impl Method for ReleaseAnimations {
            const NAME: &'static str = "Animation.releaseAnimations";
            type ReturnObject = ReleaseAnimationsReturnObject;
        }
        impl Method for ResolveAnimation {
            const NAME: &'static str = "Animation.resolveAnimation";
            type ReturnObject = ResolveAnimationReturnObject;
        }
        impl Method for SeekAnimations {
            const NAME: &'static str = "Animation.seekAnimations";
            type ReturnObject = SeekAnimationsReturnObject;
        }
        impl Method for SetPaused {
            const NAME: &'static str = "Animation.setPaused";
            type ReturnObject = SetPausedReturnObject;
        }
        impl Method for SetPlaybackRate {
            const NAME: &'static str = "Animation.setPlaybackRate";
            type ReturnObject = SetPlaybackRateReturnObject;
        }
        impl Method for SetTiming {
            const NAME: &'static str = "Animation.setTiming";
            type ReturnObject = SetTimingReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AnimationCanceledEvent {
                pub params: AnimationCanceledEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AnimationCanceledEventParams {
                #[serde(default)]
                #[serde(rename = "id")]
                pub id: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AnimationCreatedEvent {
                pub params: AnimationCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AnimationCreatedEventParams {
                #[serde(default)]
                #[serde(rename = "id")]
                pub id: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AnimationStartedEvent {
                pub params: AnimationStartedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AnimationStartedEventParams {
                #[serde(rename = "animation")]
                pub animation: super::Animation,
            }
        }
    }
    pub mod Audits {
        use super::types::*;
        use super::Network;
        use super::Page;
        use super::Runtime;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type IssueId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SameSiteCookieExclusionReason {
            #[serde(rename = "ExcludeSameSiteUnspecifiedTreatedAsLax")]
            ExcludeSameSiteUnspecifiedTreatedAsLax,
            #[serde(rename = "ExcludeSameSiteNoneInsecure")]
            ExcludeSameSiteNoneInsecure,
            #[serde(rename = "ExcludeSameSiteLax")]
            ExcludeSameSiteLax,
            #[serde(rename = "ExcludeSameSiteStrict")]
            ExcludeSameSiteStrict,
            #[serde(rename = "ExcludeInvalidSameParty")]
            ExcludeInvalidSameParty,
            #[serde(rename = "ExcludeSamePartyCrossPartyContext")]
            ExcludeSamePartyCrossPartyContext,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SameSiteCookieWarningReason {
            #[serde(rename = "WarnSameSiteUnspecifiedCrossSiteContext")]
            WarnSameSiteUnspecifiedCrossSiteContext,
            #[serde(rename = "WarnSameSiteNoneInsecure")]
            WarnSameSiteNoneInsecure,
            #[serde(rename = "WarnSameSiteUnspecifiedLaxAllowUnsafe")]
            WarnSameSiteUnspecifiedLaxAllowUnsafe,
            #[serde(rename = "WarnSameSiteStrictLaxDowngradeStrict")]
            WarnSameSiteStrictLaxDowngradeStrict,
            #[serde(rename = "WarnSameSiteStrictCrossDowngradeStrict")]
            WarnSameSiteStrictCrossDowngradeStrict,
            #[serde(rename = "WarnSameSiteStrictCrossDowngradeLax")]
            WarnSameSiteStrictCrossDowngradeLax,
            #[serde(rename = "WarnSameSiteLaxCrossDowngradeStrict")]
            WarnSameSiteLaxCrossDowngradeStrict,
            #[serde(rename = "WarnSameSiteLaxCrossDowngradeLax")]
            WarnSameSiteLaxCrossDowngradeLax,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SameSiteCookieOperation {
            #[serde(rename = "SetCookie")]
            SetCookie,
            #[serde(rename = "ReadCookie")]
            ReadCookie,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum MixedContentResolutionStatus {
            #[serde(rename = "MixedContentBlocked")]
            MixedContentBlocked,
            #[serde(rename = "MixedContentAutomaticallyUpgraded")]
            MixedContentAutomaticallyUpgraded,
            #[serde(rename = "MixedContentWarning")]
            MixedContentWarning,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum MixedContentResourceType {
            #[serde(rename = "Audio")]
            Audio,
            #[serde(rename = "Beacon")]
            Beacon,
            #[serde(rename = "CSPReport")]
            CspReport,
            #[serde(rename = "Download")]
            Download,
            #[serde(rename = "EventSource")]
            EventSource,
            #[serde(rename = "Favicon")]
            Favicon,
            #[serde(rename = "Font")]
            Font,
            #[serde(rename = "Form")]
            Form,
            #[serde(rename = "Frame")]
            Frame,
            #[serde(rename = "Image")]
            Image,
            #[serde(rename = "Import")]
            Import,
            #[serde(rename = "Manifest")]
            Manifest,
            #[serde(rename = "Ping")]
            Ping,
            #[serde(rename = "PluginData")]
            PluginData,
            #[serde(rename = "PluginResource")]
            PluginResource,
            #[serde(rename = "Prefetch")]
            Prefetch,
            #[serde(rename = "Resource")]
            Resource,
            #[serde(rename = "Script")]
            Script,
            #[serde(rename = "ServiceWorker")]
            ServiceWorker,
            #[serde(rename = "SharedWorker")]
            SharedWorker,
            #[serde(rename = "Stylesheet")]
            Stylesheet,
            #[serde(rename = "Track")]
            Track,
            #[serde(rename = "Video")]
            Video,
            #[serde(rename = "Worker")]
            Worker,
            #[serde(rename = "XMLHttpRequest")]
            XmlHttpRequest,
            #[serde(rename = "XSLT")]
            Xslt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum BlockedByResponseReason {
            #[serde(rename = "CoepFrameResourceNeedsCoepHeader")]
            CoepFrameResourceNeedsCoepHeader,
            #[serde(rename = "CoopSandboxedIFrameCannotNavigateToCoopPage")]
            CoopSandboxedIFrameCannotNavigateToCoopPage,
            #[serde(rename = "CorpNotSameOrigin")]
            CorpNotSameOrigin,
            #[serde(rename = "CorpNotSameOriginAfterDefaultedToSameOriginByCoep")]
            CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
            #[serde(rename = "CorpNotSameSite")]
            CorpNotSameSite,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum HeavyAdResolutionStatus {
            #[serde(rename = "HeavyAdBlocked")]
            HeavyAdBlocked,
            #[serde(rename = "HeavyAdWarning")]
            HeavyAdWarning,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum HeavyAdReason {
            #[serde(rename = "NetworkTotalLimit")]
            NetworkTotalLimit,
            #[serde(rename = "CpuTotalLimit")]
            CpuTotalLimit,
            #[serde(rename = "CpuPeakLimit")]
            CpuPeakLimit,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ContentSecurityPolicyViolationType {
            #[serde(rename = "kInlineViolation")]
            KInlineViolation,
            #[serde(rename = "kEvalViolation")]
            KEvalViolation,
            #[serde(rename = "kURLViolation")]
            KUrlViolation,
            #[serde(rename = "kTrustedTypesSinkViolation")]
            KTrustedTypesSinkViolation,
            #[serde(rename = "kTrustedTypesPolicyViolation")]
            KTrustedTypesPolicyViolation,
            #[serde(rename = "kWasmEvalViolation")]
            KWasmEvalViolation,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SharedArrayBufferIssueType {
            #[serde(rename = "TransferIssue")]
            TransferIssue,
            #[serde(rename = "CreationIssue")]
            CreationIssue,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum TwaQualityEnforcementViolationType {
            #[serde(rename = "kHttpError")]
            KHttpError,
            #[serde(rename = "kUnavailableOffline")]
            KUnavailableOffline,
            #[serde(rename = "kDigitalAssetLinks")]
            KDigitalAssetLinks,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum AttributionReportingIssueType {
            #[serde(rename = "PermissionPolicyDisabled")]
            PermissionPolicyDisabled,
            #[serde(rename = "InvalidAttributionSourceEventId")]
            InvalidAttributionSourceEventId,
            #[serde(rename = "InvalidAttributionData")]
            InvalidAttributionData,
            #[serde(rename = "AttributionSourceUntrustworthyOrigin")]
            AttributionSourceUntrustworthyOrigin,
            #[serde(rename = "AttributionUntrustworthyOrigin")]
            AttributionUntrustworthyOrigin,
            #[serde(rename = "AttributionTriggerDataTooLarge")]
            AttributionTriggerDataTooLarge,
            #[serde(rename = "AttributionEventSourceTriggerDataTooLarge")]
            AttributionEventSourceTriggerDataTooLarge,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum GenericIssueErrorType {
            #[serde(rename = "CrossOriginPortalPostMessageError")]
            CrossOriginPortalPostMessageError,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum InspectorIssueCode {
            #[serde(rename = "SameSiteCookieIssue")]
            SameSiteCookieIssue,
            #[serde(rename = "MixedContentIssue")]
            MixedContentIssue,
            #[serde(rename = "BlockedByResponseIssue")]
            BlockedByResponseIssue,
            #[serde(rename = "HeavyAdIssue")]
            HeavyAdIssue,
            #[serde(rename = "ContentSecurityPolicyIssue")]
            ContentSecurityPolicyIssue,
            #[serde(rename = "SharedArrayBufferIssue")]
            SharedArrayBufferIssue,
            #[serde(rename = "TrustedWebActivityIssue")]
            TrustedWebActivityIssue,
            #[serde(rename = "LowTextContrastIssue")]
            LowTextContrastIssue,
            #[serde(rename = "CorsIssue")]
            CorsIssue,
            #[serde(rename = "AttributionReportingIssue")]
            AttributionReportingIssue,
            #[serde(rename = "QuirksModeIssue")]
            QuirksModeIssue,
            #[serde(rename = "NavigatorUserAgentIssue")]
            NavigatorUserAgentIssue,
            #[serde(rename = "WasmCrossOriginModuleSharingIssue")]
            WasmCrossOriginModuleSharingIssue,
            #[serde(rename = "GenericIssue")]
            GenericIssue,
            #[serde(rename = "DeprecationIssue")]
            DeprecationIssue,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum GetEncodedResponseEncodingOption {
            #[serde(rename = "webp")]
            Webp,
            #[serde(rename = "jpeg")]
            Jpeg,
            #[serde(rename = "png")]
            Png,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AffectedCookie {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "path")]
            pub path: String,
            #[serde(default)]
            #[serde(rename = "domain")]
            pub domain: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AffectedRequest {
            #[serde(rename = "requestId")]
            pub request_id: Network::RequestId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AffectedFrame {
            #[serde(rename = "frameId")]
            pub frame_id: Page::FrameId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SameSiteCookieIssueDetails {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "cookie")]
            pub cookie: Option<AffectedCookie>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "rawCookieLine")]
            pub raw_cookie_line: Option<String>,
            #[serde(rename = "cookieWarningReasons")]
            pub cookie_warning_reasons: Vec<SameSiteCookieWarningReason>,
            #[serde(rename = "cookieExclusionReasons")]
            pub cookie_exclusion_reasons: Vec<SameSiteCookieExclusionReason>,
            #[serde(rename = "operation")]
            pub operation: SameSiteCookieOperation,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "siteForCookies")]
            pub site_for_cookies: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "cookieUrl")]
            pub cookie_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "request")]
            pub request: Option<AffectedRequest>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct MixedContentIssueDetails {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "resourceType")]
            pub resource_Type: Option<MixedContentResourceType>,
            #[serde(rename = "resolutionStatus")]
            pub resolution_status: MixedContentResolutionStatus,
            #[serde(default)]
            #[serde(rename = "insecureURL")]
            pub insecure_url: String,
            #[serde(default)]
            #[serde(rename = "mainResourceURL")]
            pub main_resource_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "request")]
            pub request: Option<AffectedRequest>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "frame")]
            pub frame: Option<AffectedFrame>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct BlockedByResponseIssueDetails {
            #[serde(rename = "request")]
            pub request: AffectedRequest,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "parentFrame")]
            pub parent_frame: Option<AffectedFrame>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "blockedFrame")]
            pub blocked_frame: Option<AffectedFrame>,
            #[serde(rename = "reason")]
            pub reason: BlockedByResponseReason,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct HeavyAdIssueDetails {
            #[serde(rename = "resolution")]
            pub resolution: HeavyAdResolutionStatus,
            #[serde(rename = "reason")]
            pub reason: HeavyAdReason,
            #[serde(rename = "frame")]
            pub frame: AffectedFrame,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SourceCodeLocation {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "scriptId")]
            pub script_id: Option<Runtime::ScriptId>,
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(default)]
            #[serde(rename = "lineNumber")]
            pub line_number: JsUInt,
            #[serde(default)]
            #[serde(rename = "columnNumber")]
            pub column_number: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ContentSecurityPolicyIssueDetails {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "blockedURL")]
            pub blocked_url: Option<String>,
            #[serde(default)]
            #[serde(rename = "violatedDirective")]
            pub violated_directive: String,
            #[serde(default)]
            #[serde(rename = "isReportOnly")]
            pub is_report_only: bool,
            #[serde(rename = "contentSecurityPolicyViolationType")]
            pub content_security_policy_violation_Type: ContentSecurityPolicyViolationType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "frameAncestor")]
            pub frame_ancestor: Option<AffectedFrame>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "sourceCodeLocation")]
            pub source_code_location: Option<SourceCodeLocation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "violatingNodeId")]
            pub violating_node_id: Option<DOM::BackendNodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SharedArrayBufferIssueDetails {
            #[serde(rename = "sourceCodeLocation")]
            pub source_code_location: SourceCodeLocation,
            #[serde(default)]
            #[serde(rename = "isWarning")]
            pub is_warning: bool,
            #[serde(rename = "type")]
            pub Type: SharedArrayBufferIssueType,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TrustedWebActivityIssueDetails {
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(rename = "violationType")]
            pub violation_Type: TwaQualityEnforcementViolationType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "httpStatusCode")]
            pub http_status_code: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "packageName")]
            pub package_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "signature")]
            pub signature: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct LowTextContrastIssueDetails {
            #[serde(rename = "violatingNodeId")]
            pub violating_node_id: DOM::BackendNodeId,
            #[serde(default)]
            #[serde(rename = "violatingNodeSelector")]
            pub violating_node_selector: String,
            #[serde(default)]
            #[serde(rename = "contrastRatio")]
            pub contrast_ratio: JsFloat,
            #[serde(default)]
            #[serde(rename = "thresholdAA")]
            pub threshold_aa: JsFloat,
            #[serde(default)]
            #[serde(rename = "thresholdAAA")]
            pub threshold_aaa: JsFloat,
            #[serde(default)]
            #[serde(rename = "fontSize")]
            pub font_size: String,
            #[serde(default)]
            #[serde(rename = "fontWeight")]
            pub font_weight: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CorsIssueDetails {
            #[serde(rename = "corsErrorStatus")]
            pub cors_error_status: Network::CorsErrorStatus,
            #[serde(default)]
            #[serde(rename = "isWarning")]
            pub is_warning: bool,
            #[serde(rename = "request")]
            pub request: AffectedRequest,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "location")]
            pub location: Option<SourceCodeLocation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "initiatorOrigin")]
            pub initiator_origin: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "resourceIPAddressSpace")]
            pub resource_ip_address_space: Option<Network::IPAddressSpace>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "clientSecurityState")]
            pub client_security_state: Option<Network::ClientSecurityState>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AttributionReportingIssueDetails {
            #[serde(rename = "violationType")]
            pub violation_Type: AttributionReportingIssueType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "frame")]
            pub frame: Option<AffectedFrame>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "request")]
            pub request: Option<AffectedRequest>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "violatingNodeId")]
            pub violating_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "invalidParameter")]
            pub invalid_parameter: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct QuirksModeIssueDetails {
            #[serde(default)]
            #[serde(rename = "isLimitedQuirksMode")]
            pub is_limited_quirks_mode: bool,
            #[serde(rename = "documentNodeId")]
            pub document_node_id: DOM::BackendNodeId,
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(rename = "frameId")]
            pub frame_id: Page::FrameId,
            #[serde(rename = "loaderId")]
            pub loader_id: Network::LoaderId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct NavigatorUserAgentIssueDetails {
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "location")]
            pub location: Option<SourceCodeLocation>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct WasmCrossOriginModuleSharingIssueDetails {
            #[serde(default)]
            #[serde(rename = "wasmModuleUrl")]
            pub wasm_module_url: String,
            #[serde(default)]
            #[serde(rename = "sourceOrigin")]
            pub source_origin: String,
            #[serde(default)]
            #[serde(rename = "targetOrigin")]
            pub target_origin: String,
            #[serde(default)]
            #[serde(rename = "isWarning")]
            pub is_warning: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GenericIssueDetails {
            #[serde(rename = "errorType")]
            pub error_Type: GenericIssueErrorType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "frameId")]
            pub frame_id: Option<Page::FrameId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DeprecationIssueDetails {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "affectedFrame")]
            pub affected_frame: Option<AffectedFrame>,
            #[serde(rename = "sourceCodeLocation")]
            pub source_code_location: SourceCodeLocation,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "message")]
            pub message: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct InspectorIssueDetails {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "sameSiteCookieIssueDetails")]
            pub same_site_cookie_issue_details: Option<SameSiteCookieIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "mixedContentIssueDetails")]
            pub mixed_content_issue_details: Option<MixedContentIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "blockedByResponseIssueDetails")]
            pub blocked_by_response_issue_details: Option<BlockedByResponseIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "heavyAdIssueDetails")]
            pub heavy_ad_issue_details: Option<HeavyAdIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "contentSecurityPolicyIssueDetails")]
            pub content_security_policy_issue_details: Option<ContentSecurityPolicyIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "sharedArrayBufferIssueDetails")]
            pub shared_array_buffer_issue_details: Option<SharedArrayBufferIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "twaQualityEnforcementDetails")]
            pub twa_quality_enforcement_details: Option<TrustedWebActivityIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "lowTextContrastIssueDetails")]
            pub low_text_contrast_issue_details: Option<LowTextContrastIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "corsIssueDetails")]
            pub cors_issue_details: Option<CorsIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "attributionReportingIssueDetails")]
            pub attribution_reporting_issue_details: Option<AttributionReportingIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "quirksModeIssueDetails")]
            pub quirks_mode_issue_details: Option<QuirksModeIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "navigatorUserAgentIssueDetails")]
            pub navigator_user_agent_issue_details: Option<NavigatorUserAgentIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "wasmCrossOriginModuleSharingIssue")]
            pub wasm_cross_origin_module_sharing_issue:
                Option<WasmCrossOriginModuleSharingIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "genericIssueDetails")]
            pub generic_issue_details: Option<GenericIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "deprecationIssueDetails")]
            pub deprecation_issue_details: Option<DeprecationIssueDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct InspectorIssue {
            #[serde(rename = "code")]
            pub code: InspectorIssueCode,
            #[serde(rename = "details")]
            pub details: InspectorIssueDetails,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "issueId")]
            pub issue_id: Option<IssueId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetEncodedResponse {
            #[serde(rename = "requestId")]
            pub request_id: Network::RequestId,
            #[serde(rename = "encoding")]
            pub encoding: GetEncodedResponseEncodingOption,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "quality")]
            pub quality: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "sizeOnly")]
            pub size_only: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CheckContrast {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "reportAAA")]
            pub report_aaa: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetEncodedResponseReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "body")]
            pub body: Option<String>,
            #[serde(default)]
            #[serde(rename = "originalSize")]
            pub original_size: JsUInt,
            #[serde(default)]
            #[serde(rename = "encodedSize")]
            pub encoded_size: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CheckContrastReturnObject {}
        impl Method for GetEncodedResponse {
            const NAME: &'static str = "Audits.getEncodedResponse";
            type ReturnObject = GetEncodedResponseReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Audits.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Audits.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for CheckContrast {
            const NAME: &'static str = "Audits.checkContrast";
            type ReturnObject = CheckContrastReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct IssueAddedEvent {
                pub params: IssueAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct IssueAddedEventParams {
                #[serde(rename = "issue")]
                pub issue: super::InspectorIssue,
            }
        }
    }
    pub mod BackgroundService {
        use super::types::*;
        use super::Network;
        use super::ServiceWorker;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ServiceName {
            #[serde(rename = "backgroundFetch")]
            BackgroundFetch,
            #[serde(rename = "backgroundSync")]
            BackgroundSync,
            #[serde(rename = "pushMessaging")]
            PushMessaging,
            #[serde(rename = "notifications")]
            Notifications,
            #[serde(rename = "paymentHandler")]
            PaymentHandler,
            #[serde(rename = "periodicBackgroundSync")]
            PeriodicBackgroundSync,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct EventMetadata {
            #[serde(default)]
            #[serde(rename = "key")]
            pub key: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct BackgroundServiceEvent {
            #[serde(rename = "timestamp")]
            pub timestamp: Network::TimeSinceEpoch,
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: String,
            #[serde(rename = "serviceWorkerRegistrationId")]
            pub service_worker_registration_id: ServiceWorker::RegistrationID,
            #[serde(rename = "service")]
            pub service: ServiceName,
            #[serde(default)]
            #[serde(rename = "eventName")]
            pub event_name: String,
            #[serde(default)]
            #[serde(rename = "instanceId")]
            pub instance_id: String,
            #[serde(rename = "eventMetadata")]
            pub event_metadata: Vec<EventMetadata>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StartObserving {
            #[serde(rename = "service")]
            pub service: ServiceName,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StopObserving {
            #[serde(rename = "service")]
            pub service: ServiceName,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetRecording {
            #[serde(default)]
            #[serde(rename = "shouldRecord")]
            pub should_record: bool,
            #[serde(rename = "service")]
            pub service: ServiceName,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ClearEvents {
            #[serde(rename = "service")]
            pub service: ServiceName,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartObservingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopObservingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetRecordingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearEventsReturnObject {}
        impl Method for StartObserving {
            const NAME: &'static str = "BackgroundService.startObserving";
            type ReturnObject = StartObservingReturnObject;
        }
        impl Method for StopObserving {
            const NAME: &'static str = "BackgroundService.stopObserving";
            type ReturnObject = StopObservingReturnObject;
        }
        impl Method for SetRecording {
            const NAME: &'static str = "BackgroundService.setRecording";
            type ReturnObject = SetRecordingReturnObject;
        }
        impl Method for ClearEvents {
            const NAME: &'static str = "BackgroundService.clearEvents";
            type ReturnObject = ClearEventsReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RecordingStateChangedEvent {
                pub params: RecordingStateChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RecordingStateChangedEventParams {
                #[serde(default)]
                #[serde(rename = "isRecording")]
                pub is_recording: bool,
                #[serde(rename = "service")]
                pub service: super::ServiceName,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct BackgroundServiceEventReceivedEvent {
                pub params: BackgroundServiceEventReceivedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct BackgroundServiceEventReceivedEventParams {
                #[serde(rename = "backgroundServiceEvent")]
                pub background_service_event: super::BackgroundServiceEvent,
            }
        }
    }
    pub mod Browser {
        use super::types::*;
        use super::Target;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type BrowserContextID = String;
        pub type WindowID = JsUInt;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum WindowState {
            #[serde(rename = "normal")]
            Normal,
            #[serde(rename = "minimized")]
            Minimized,
            #[serde(rename = "maximized")]
            Maximized,
            #[serde(rename = "fullscreen")]
            Fullscreen,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum PermissionType {
            #[serde(rename = "accessibilityEvents")]
            AccessibilityEvents,
            #[serde(rename = "audioCapture")]
            AudioCapture,
            #[serde(rename = "backgroundSync")]
            BackgroundSync,
            #[serde(rename = "backgroundFetch")]
            BackgroundFetch,
            #[serde(rename = "clipboardReadWrite")]
            ClipboardReadWrite,
            #[serde(rename = "clipboardSanitizedWrite")]
            ClipboardSanitizedWrite,
            #[serde(rename = "displayCapture")]
            DisplayCapture,
            #[serde(rename = "durableStorage")]
            DurableStorage,
            #[serde(rename = "flash")]
            Flash,
            #[serde(rename = "geolocation")]
            Geolocation,
            #[serde(rename = "midi")]
            Midi,
            #[serde(rename = "midiSysex")]
            MidiSysex,
            #[serde(rename = "nfc")]
            Nfc,
            #[serde(rename = "notifications")]
            Notifications,
            #[serde(rename = "paymentHandler")]
            PaymentHandler,
            #[serde(rename = "periodicBackgroundSync")]
            PeriodicBackgroundSync,
            #[serde(rename = "protectedMediaIdentifier")]
            ProtectedMediaIdentifier,
            #[serde(rename = "sensors")]
            Sensors,
            #[serde(rename = "videoCapture")]
            VideoCapture,
            #[serde(rename = "videoCapturePanTiltZoom")]
            VideoCapturePanTiltZoom,
            #[serde(rename = "idleDetection")]
            IdleDetection,
            #[serde(rename = "wakeLockScreen")]
            WakeLockScreen,
            #[serde(rename = "wakeLockSystem")]
            WakeLockSystem,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum PermissionSetting {
            #[serde(rename = "granted")]
            Granted,
            #[serde(rename = "denied")]
            Denied,
            #[serde(rename = "prompt")]
            Prompt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum BrowserCommandId {
            #[serde(rename = "openTabSearch")]
            OpenTabSearch,
            #[serde(rename = "closeTabSearch")]
            CloseTabSearch,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SetDownloadBehaviorBehaviorOption {
            #[serde(rename = "deny")]
            Deny,
            #[serde(rename = "allow")]
            Allow,
            #[serde(rename = "allowAndName")]
            AllowAndName,
            #[serde(rename = "default")]
            Default,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum DownloadProgressEventStateOption {
            #[serde(rename = "inProgress")]
            InProgress,
            #[serde(rename = "completed")]
            Completed,
            #[serde(rename = "canceled")]
            Canceled,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Bounds {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "left")]
            pub left: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "top")]
            pub top: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "width")]
            pub width: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "height")]
            pub height: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "windowState")]
            pub window_state: Option<WindowState>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PermissionDescriptor {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "sysex")]
            pub sysex: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "userVisibleOnly")]
            pub user_visible_only: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "allowWithoutSanitization")]
            pub allow_without_sanitization: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "panTiltZoom")]
            pub pan_tilt_zoom: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Bucket {
            #[serde(default)]
            #[serde(rename = "low")]
            pub low: JsUInt,
            #[serde(default)]
            #[serde(rename = "high")]
            pub high: JsUInt,
            #[serde(default)]
            #[serde(rename = "count")]
            pub count: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Histogram {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "sum")]
            pub sum: JsUInt,
            #[serde(default)]
            #[serde(rename = "count")]
            pub count: JsUInt,
            #[serde(rename = "buckets")]
            pub buckets: Vec<Bucket>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetPermission {
            #[serde(rename = "permission")]
            pub permission: PermissionDescriptor,
            #[serde(rename = "setting")]
            pub setting: PermissionSetting,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "browserContextId")]
            pub browser_context_id: Option<BrowserContextID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GrantPermissions {
            #[serde(rename = "permissions")]
            pub permissions: Vec<PermissionType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "browserContextId")]
            pub browser_context_id: Option<BrowserContextID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ResetPermissions {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "browserContextId")]
            pub browser_context_id: Option<BrowserContextID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetDownloadBehavior {
            #[serde(rename = "behavior")]
            pub behavior: SetDownloadBehaviorBehaviorOption,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "browserContextId")]
            pub browser_context_id: Option<BrowserContextID>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "downloadPath")]
            pub download_path: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "eventsEnabled")]
            pub events_enabled: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CancelDownload {
            #[serde(default)]
            #[serde(rename = "guid")]
            pub guid: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "browserContextId")]
            pub browser_context_id: Option<BrowserContextID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Close(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Crash(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CrashGpuProcess(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetVersion(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetBrowserCommandLine(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetHistograms {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "query")]
            pub query: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "delta")]
            pub delta: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetHistogram {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "delta")]
            pub delta: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetWindowBounds {
            #[serde(rename = "windowId")]
            pub window_id: WindowID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetWindowForTarget {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "targetId")]
            pub target_id: Option<Target::TargetID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetWindowBounds {
            #[serde(rename = "windowId")]
            pub window_id: WindowID,
            #[serde(rename = "bounds")]
            pub bounds: Bounds,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetDockTile {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "badgeLabel")]
            pub badge_label: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "image")]
            pub image: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ExecuteBrowserCommand {
            #[serde(rename = "commandId")]
            pub command_id: BrowserCommandId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPermissionReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GrantPermissionsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResetPermissionsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDownloadBehaviorReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CancelDownloadReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CloseReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CrashReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CrashGpuProcessReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetVersionReturnObject {
            #[serde(default)]
            #[serde(rename = "protocolVersion")]
            pub protocol_version: String,
            #[serde(default)]
            #[serde(rename = "product")]
            pub product: String,
            #[serde(default)]
            #[serde(rename = "revision")]
            pub revision: String,
            #[serde(default)]
            #[serde(rename = "userAgent")]
            pub user_agent: String,
            #[serde(default)]
            #[serde(rename = "jsVersion")]
            pub js_version: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetBrowserCommandLineReturnObject {
            #[serde(rename = "arguments")]
            pub arguments: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetHistogramsReturnObject {
            #[serde(rename = "histograms")]
            pub histograms: Vec<Histogram>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetHistogramReturnObject {
            #[serde(rename = "histogram")]
            pub histogram: Histogram,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetWindowBoundsReturnObject {
            #[serde(rename = "bounds")]
            pub bounds: Bounds,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetWindowForTargetReturnObject {
            #[serde(rename = "windowId")]
            pub window_id: WindowID,
            #[serde(rename = "bounds")]
            pub bounds: Bounds,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetWindowBoundsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDockTileReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ExecuteBrowserCommandReturnObject {}
        impl Method for SetPermission {
            const NAME: &'static str = "Browser.setPermission";
            type ReturnObject = SetPermissionReturnObject;
        }
        impl Method for GrantPermissions {
            const NAME: &'static str = "Browser.grantPermissions";
            type ReturnObject = GrantPermissionsReturnObject;
        }
        impl Method for ResetPermissions {
            const NAME: &'static str = "Browser.resetPermissions";
            type ReturnObject = ResetPermissionsReturnObject;
        }
        impl Method for SetDownloadBehavior {
            const NAME: &'static str = "Browser.setDownloadBehavior";
            type ReturnObject = SetDownloadBehaviorReturnObject;
        }
        impl Method for CancelDownload {
            const NAME: &'static str = "Browser.cancelDownload";
            type ReturnObject = CancelDownloadReturnObject;
        }
        impl Method for Close {
            const NAME: &'static str = "Browser.close";
            type ReturnObject = CloseReturnObject;
        }
        impl Method for Crash {
            const NAME: &'static str = "Browser.crash";
            type ReturnObject = CrashReturnObject;
        }
        impl Method for CrashGpuProcess {
            const NAME: &'static str = "Browser.crashGpuProcess";
            type ReturnObject = CrashGpuProcessReturnObject;
        }
        impl Method for GetVersion {
            const NAME: &'static str = "Browser.getVersion";
            type ReturnObject = GetVersionReturnObject;
        }
        impl Method for GetBrowserCommandLine {
            const NAME: &'static str = "Browser.getBrowserCommandLine";
            type ReturnObject = GetBrowserCommandLineReturnObject;
        }
        impl Method for GetHistograms {
            const NAME: &'static str = "Browser.getHistograms";
            type ReturnObject = GetHistogramsReturnObject;
        }
        impl Method for GetHistogram {
            const NAME: &'static str = "Browser.getHistogram";
            type ReturnObject = GetHistogramReturnObject;
        }
        impl Method for GetWindowBounds {
            const NAME: &'static str = "Browser.getWindowBounds";
            type ReturnObject = GetWindowBoundsReturnObject;
        }
        impl Method for GetWindowForTarget {
            const NAME: &'static str = "Browser.getWindowForTarget";
            type ReturnObject = GetWindowForTargetReturnObject;
        }
        impl Method for SetWindowBounds {
            const NAME: &'static str = "Browser.setWindowBounds";
            type ReturnObject = SetWindowBoundsReturnObject;
        }
        impl Method for SetDockTile {
            const NAME: &'static str = "Browser.setDockTile";
            type ReturnObject = SetDockTileReturnObject;
        }
        impl Method for ExecuteBrowserCommand {
            const NAME: &'static str = "Browser.executeBrowserCommand";
            type ReturnObject = ExecuteBrowserCommandReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DownloadWillBeginEvent {
                pub params: DownloadWillBeginEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DownloadWillBeginEventParams {
                #[serde(rename = "frameId")]
                pub frame_id: super::super::Page::FrameId,
                #[serde(default)]
                #[serde(rename = "guid")]
                pub guid: String,
                #[serde(default)]
                #[serde(rename = "url")]
                pub url: String,
                #[serde(default)]
                #[serde(rename = "suggestedFilename")]
                pub suggested_filename: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DownloadProgressEvent {
                pub params: DownloadProgressEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DownloadProgressEventParams {
                #[serde(default)]
                #[serde(rename = "guid")]
                pub guid: String,
                #[serde(default)]
                #[serde(rename = "totalBytes")]
                pub total_bytes: JsFloat,
                #[serde(default)]
                #[serde(rename = "receivedBytes")]
                pub received_bytes: JsFloat,
                #[serde(rename = "state")]
                pub state: super::DownloadProgressEventStateOption,
            }
        }
    }
    pub mod CSS {
        use super::types::*;
        use super::Page;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type StyleSheetId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum StyleSheetOrigin {
            #[serde(rename = "injected")]
            Injected,
            #[serde(rename = "user-agent")]
            UserAgent,
            #[serde(rename = "inspector")]
            Inspector,
            #[serde(rename = "regular")]
            Regular,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum CssMediaSource {
            #[serde(rename = "mediaRule")]
            MediaRule,
            #[serde(rename = "importRule")]
            ImportRule,
            #[serde(rename = "linkedSheet")]
            LinkedSheet,
            #[serde(rename = "inlineSheet")]
            InlineSheet,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PseudoElementMatches {
            #[serde(rename = "pseudoType")]
            pub pseudo_Type: DOM::PseudoType,
            #[serde(rename = "matches")]
            pub matches: Vec<RuleMatch>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct InheritedStyleEntry {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "inlineStyle")]
            pub inline_style: Option<CSSStyle>,
            #[serde(rename = "matchedCSSRules")]
            pub matched_css_rules: Vec<RuleMatch>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RuleMatch {
            #[serde(rename = "rule")]
            pub rule: CSSRule,
            #[serde(default)]
            #[serde(rename = "matchingSelectors")]
            pub matching_selectors: Vec<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Value {
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "range")]
            pub range: Option<SourceRange>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SelectorList {
            #[serde(rename = "selectors")]
            pub selectors: Vec<Value>,
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CSSStyleSheetHeader {
            #[serde(rename = "styleSheetId")]
            pub style_sheet_id: StyleSheetId,
            #[serde(rename = "frameId")]
            pub frame_id: Page::FrameId,
            #[serde(default)]
            #[serde(rename = "sourceURL")]
            pub source_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "sourceMapURL")]
            pub source_map_url: Option<String>,
            #[serde(rename = "origin")]
            pub origin: StyleSheetOrigin,
            #[serde(default)]
            #[serde(rename = "title")]
            pub title: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "ownerNode")]
            pub owner_node: Option<DOM::BackendNodeId>,
            #[serde(default)]
            #[serde(rename = "disabled")]
            pub disabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "hasSourceURL")]
            pub has_source_url: Option<bool>,
            #[serde(default)]
            #[serde(rename = "isInline")]
            pub is_inline: bool,
            #[serde(default)]
            #[serde(rename = "isMutable")]
            pub is_mutable: bool,
            #[serde(default)]
            #[serde(rename = "isConstructed")]
            pub is_constructed: bool,
            #[serde(default)]
            #[serde(rename = "startLine")]
            pub start_line: JsFloat,
            #[serde(default)]
            #[serde(rename = "startColumn")]
            pub start_column: JsFloat,
            #[serde(default)]
            #[serde(rename = "length")]
            pub length: JsFloat,
            #[serde(default)]
            #[serde(rename = "endLine")]
            pub end_line: JsFloat,
            #[serde(default)]
            #[serde(rename = "endColumn")]
            pub end_column: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CSSRule {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "styleSheetId")]
            pub style_sheet_id: Option<StyleSheetId>,
            #[serde(rename = "selectorList")]
            pub selector_list: SelectorList,
            #[serde(rename = "origin")]
            pub origin: StyleSheetOrigin,
            #[serde(rename = "style")]
            pub style: CSSStyle,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "media")]
            pub media: Option<Vec<CSSMedia>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "containerQueries")]
            pub container_queries: Option<Vec<CSSContainerQuery>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RuleUsage {
            #[serde(rename = "styleSheetId")]
            pub style_sheet_id: StyleSheetId,
            #[serde(default)]
            #[serde(rename = "startOffset")]
            pub start_offset: JsFloat,
            #[serde(default)]
            #[serde(rename = "endOffset")]
            pub end_offset: JsFloat,
            #[serde(default)]
            #[serde(rename = "used")]
            pub used: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SourceRange {
            #[serde(default)]
            #[serde(rename = "startLine")]
            pub start_line: JsUInt,
            #[serde(default)]
            #[serde(rename = "startColumn")]
            pub start_column: JsUInt,
            #[serde(default)]
            #[serde(rename = "endLine")]
            pub end_line: JsUInt,
            #[serde(default)]
            #[serde(rename = "endColumn")]
            pub end_column: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ShorthandEntry {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "important")]
            pub important: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CSSComputedStyleProperty {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CSSStyle {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "styleSheetId")]
            pub style_sheet_id: Option<StyleSheetId>,
            #[serde(rename = "cssProperties")]
            pub css_properties: Vec<CSSProperty>,
            #[serde(rename = "shorthandEntries")]
            pub shorthand_entries: Vec<ShorthandEntry>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "cssText")]
            pub css_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "range")]
            pub range: Option<SourceRange>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CSSProperty {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "important")]
            pub important: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "implicit")]
            pub implicit: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "parsedOk")]
            pub parsed_ok: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "disabled")]
            pub disabled: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "range")]
            pub range: Option<SourceRange>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CSSMedia {
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: String,
            #[serde(rename = "source")]
            pub source: CssMediaSource,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "sourceURL")]
            pub source_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "range")]
            pub range: Option<SourceRange>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "styleSheetId")]
            pub style_sheet_id: Option<StyleSheetId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "mediaList")]
            pub media_list: Option<Vec<MediaQuery>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct MediaQuery {
            #[serde(rename = "expressions")]
            pub expressions: Vec<MediaQueryExpression>,
            #[serde(default)]
            #[serde(rename = "active")]
            pub active: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct MediaQueryExpression {
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: JsFloat,
            #[serde(default)]
            #[serde(rename = "unit")]
            pub unit: String,
            #[serde(default)]
            #[serde(rename = "feature")]
            pub feature: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "valueRange")]
            pub value_range: Option<SourceRange>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "computedLength")]
            pub computed_length: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CSSContainerQuery {
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "range")]
            pub range: Option<SourceRange>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "styleSheetId")]
            pub style_sheet_id: Option<StyleSheetId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PlatformFontUsage {
            #[serde(default)]
            #[serde(rename = "familyName")]
            pub family_name: String,
            #[serde(default)]
            #[serde(rename = "isCustomFont")]
            pub is_custom_font: bool,
            #[serde(default)]
            #[serde(rename = "glyphCount")]
            pub glyph_count: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct FontVariationAxis {
            #[serde(default)]
            #[serde(rename = "tag")]
            pub tag: String,
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "minValue")]
            pub min_value: JsFloat,
            #[serde(default)]
            #[serde(rename = "maxValue")]
            pub max_value: JsFloat,
            #[serde(default)]
            #[serde(rename = "defaultValue")]
            pub default_value: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct FontFace {
            #[serde(default)]
            #[serde(rename = "fontFamily")]
            pub font_family: String,
            #[serde(default)]
            #[serde(rename = "fontStyle")]
            pub font_style: String,
            #[serde(default)]
            #[serde(rename = "fontVariant")]
            pub font_variant: String,
            #[serde(default)]
            #[serde(rename = "fontWeight")]
            pub font_weight: String,
            #[serde(default)]
            #[serde(rename = "fontStretch")]
            pub font_stretch: String,
            #[serde(default)]
            #[serde(rename = "unicodeRange")]
            pub unicode_range: String,
            #[serde(default)]
            #[serde(rename = "src")]
            pub src: String,
            #[serde(default)]
            #[serde(rename = "platformFontFamily")]
            pub platform_font_family: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "fontVariationAxes")]
            pub font_variation_axes: Option<Vec<FontVariationAxis>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CSSKeyframesRule {
            #[serde(rename = "animationName")]
            pub animation_name: Value,
            #[serde(rename = "keyframes")]
            pub keyframes: Vec<CSSKeyframeRule>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CSSKeyframeRule {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "styleSheetId")]
            pub style_sheet_id: Option<StyleSheetId>,
            #[serde(rename = "origin")]
            pub origin: StyleSheetOrigin,
            #[serde(rename = "keyText")]
            pub key_text: Value,
            #[serde(rename = "style")]
            pub style: CSSStyle,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StyleDeclarationEdit {
            #[serde(rename = "styleSheetId")]
            pub style_sheet_id: StyleSheetId,
            #[serde(rename = "range")]
            pub range: SourceRange,
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AddRule {
            #[serde(rename = "styleSheetId")]
            pub style_sheet_id: StyleSheetId,
            #[serde(default)]
            #[serde(rename = "ruleText")]
            pub rule_text: String,
            #[serde(rename = "location")]
            pub location: SourceRange,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CollectClassNames {
            #[serde(rename = "styleSheetId")]
            pub style_sheet_id: StyleSheetId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CreateStyleSheet {
            #[serde(rename = "frameId")]
            pub frame_id: Page::FrameId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ForcePseudoState {
            #[serde(rename = "nodeId")]
            pub node_id: DOM::NodeId,
            #[serde(default)]
            #[serde(rename = "forcedPseudoClasses")]
            pub forced_pseudo_classes: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetBackgroundColors {
            #[serde(rename = "nodeId")]
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetComputedStyleForNode {
            #[serde(rename = "nodeId")]
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetInlineStylesForNode {
            #[serde(rename = "nodeId")]
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetMatchedStylesForNode {
            #[serde(rename = "nodeId")]
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetMediaQueries(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetPlatformFontsForNode {
            #[serde(rename = "nodeId")]
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetStyleSheetText {
            #[serde(rename = "styleSheetId")]
            pub style_sheet_id: StyleSheetId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TrackComputedStyleUpdates {
            #[serde(rename = "propertiesToTrack")]
            pub properties_to_track: Vec<CSSComputedStyleProperty>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakeComputedStyleUpdates(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetEffectivePropertyValueForNode {
            #[serde(rename = "nodeId")]
            pub node_id: DOM::NodeId,
            #[serde(default)]
            #[serde(rename = "propertyName")]
            pub property_name: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetKeyframeKey {
            #[serde(rename = "styleSheetId")]
            pub style_sheet_id: StyleSheetId,
            #[serde(rename = "range")]
            pub range: SourceRange,
            #[serde(default)]
            #[serde(rename = "keyText")]
            pub key_text: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetMediaText {
            #[serde(rename = "styleSheetId")]
            pub style_sheet_id: StyleSheetId,
            #[serde(rename = "range")]
            pub range: SourceRange,
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetContainerQueryText {
            #[serde(rename = "styleSheetId")]
            pub style_sheet_id: StyleSheetId,
            #[serde(rename = "range")]
            pub range: SourceRange,
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetRuleSelector {
            #[serde(rename = "styleSheetId")]
            pub style_sheet_id: StyleSheetId,
            #[serde(rename = "range")]
            pub range: SourceRange,
            #[serde(default)]
            #[serde(rename = "selector")]
            pub selector: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetStyleSheetText {
            #[serde(rename = "styleSheetId")]
            pub style_sheet_id: StyleSheetId,
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetStyleTexts {
            #[serde(rename = "edits")]
            pub edits: Vec<StyleDeclarationEdit>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartRuleUsageTracking(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopRuleUsageTracking(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakeCoverageDelta(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetLocalFontsEnabled {
            #[serde(default)]
            #[serde(rename = "enabled")]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AddRuleReturnObject {
            #[serde(rename = "rule")]
            pub rule: CSSRule,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CollectClassNamesReturnObject {
            #[serde(rename = "classNames")]
            pub class_names: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CreateStyleSheetReturnObject {
            #[serde(rename = "styleSheetId")]
            pub style_sheet_id: StyleSheetId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ForcePseudoStateReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetBackgroundColorsReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backgroundColors")]
            pub background_colors: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "computedFontSize")]
            pub computed_font_size: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "computedFontWeight")]
            pub computed_font_weight: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetComputedStyleForNodeReturnObject {
            #[serde(rename = "computedStyle")]
            pub computed_style: Vec<CSSComputedStyleProperty>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetInlineStylesForNodeReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "inlineStyle")]
            pub inline_style: Option<CSSStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "attributesStyle")]
            pub attributes_style: Option<CSSStyle>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetMatchedStylesForNodeReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "inlineStyle")]
            pub inline_style: Option<CSSStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "attributesStyle")]
            pub attributes_style: Option<CSSStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "matchedCSSRules")]
            pub matched_css_rules: Option<Vec<RuleMatch>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "pseudoElements")]
            pub pseudo_elements: Option<Vec<PseudoElementMatches>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "inherited")]
            pub inherited: Option<Vec<InheritedStyleEntry>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "cssKeyframesRules")]
            pub css_keyframes_rules: Option<Vec<CSSKeyframesRule>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetMediaQueriesReturnObject {
            #[serde(rename = "medias")]
            pub medias: Vec<CSSMedia>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetPlatformFontsForNodeReturnObject {
            #[serde(rename = "fonts")]
            pub fonts: Vec<PlatformFontUsage>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetStyleSheetTextReturnObject {
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TrackComputedStyleUpdatesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TakeComputedStyleUpdatesReturnObject {
            #[serde(rename = "nodeIds")]
            pub node_ids: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetEffectivePropertyValueForNodeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetKeyframeKeyReturnObject {
            #[serde(rename = "keyText")]
            pub key_text: Value,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetMediaTextReturnObject {
            #[serde(rename = "media")]
            pub media: CSSMedia,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetContainerQueryTextReturnObject {
            #[serde(rename = "containerQuery")]
            pub container_query: CSSContainerQuery,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetRuleSelectorReturnObject {
            #[serde(rename = "selectorList")]
            pub selector_list: SelectorList,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetStyleSheetTextReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "sourceMapURL")]
            pub source_map_url: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetStyleTextsReturnObject {
            #[serde(rename = "styles")]
            pub styles: Vec<CSSStyle>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartRuleUsageTrackingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StopRuleUsageTrackingReturnObject {
            #[serde(rename = "ruleUsage")]
            pub rule_usage: Vec<RuleUsage>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TakeCoverageDeltaReturnObject {
            #[serde(rename = "coverage")]
            pub coverage: Vec<RuleUsage>,
            #[serde(default)]
            #[serde(rename = "timestamp")]
            pub timestamp: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetLocalFontsEnabledReturnObject {}
        impl Method for AddRule {
            const NAME: &'static str = "CSS.addRule";
            type ReturnObject = AddRuleReturnObject;
        }
        impl Method for CollectClassNames {
            const NAME: &'static str = "CSS.collectClassNames";
            type ReturnObject = CollectClassNamesReturnObject;
        }
        impl Method for CreateStyleSheet {
            const NAME: &'static str = "CSS.createStyleSheet";
            type ReturnObject = CreateStyleSheetReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "CSS.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "CSS.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for ForcePseudoState {
            const NAME: &'static str = "CSS.forcePseudoState";
            type ReturnObject = ForcePseudoStateReturnObject;
        }
        impl Method for GetBackgroundColors {
            const NAME: &'static str = "CSS.getBackgroundColors";
            type ReturnObject = GetBackgroundColorsReturnObject;
        }
        impl Method for GetComputedStyleForNode {
            const NAME: &'static str = "CSS.getComputedStyleForNode";
            type ReturnObject = GetComputedStyleForNodeReturnObject;
        }
        impl Method for GetInlineStylesForNode {
            const NAME: &'static str = "CSS.getInlineStylesForNode";
            type ReturnObject = GetInlineStylesForNodeReturnObject;
        }
        impl Method for GetMatchedStylesForNode {
            const NAME: &'static str = "CSS.getMatchedStylesForNode";
            type ReturnObject = GetMatchedStylesForNodeReturnObject;
        }
        impl Method for GetMediaQueries {
            const NAME: &'static str = "CSS.getMediaQueries";
            type ReturnObject = GetMediaQueriesReturnObject;
        }
        impl Method for GetPlatformFontsForNode {
            const NAME: &'static str = "CSS.getPlatformFontsForNode";
            type ReturnObject = GetPlatformFontsForNodeReturnObject;
        }
        impl Method for GetStyleSheetText {
            const NAME: &'static str = "CSS.getStyleSheetText";
            type ReturnObject = GetStyleSheetTextReturnObject;
        }
        impl Method for TrackComputedStyleUpdates {
            const NAME: &'static str = "CSS.trackComputedStyleUpdates";
            type ReturnObject = TrackComputedStyleUpdatesReturnObject;
        }
        impl Method for TakeComputedStyleUpdates {
            const NAME: &'static str = "CSS.takeComputedStyleUpdates";
            type ReturnObject = TakeComputedStyleUpdatesReturnObject;
        }
        impl Method for SetEffectivePropertyValueForNode {
            const NAME: &'static str = "CSS.setEffectivePropertyValueForNode";
            type ReturnObject = SetEffectivePropertyValueForNodeReturnObject;
        }
        impl Method for SetKeyframeKey {
            const NAME: &'static str = "CSS.setKeyframeKey";
            type ReturnObject = SetKeyframeKeyReturnObject;
        }
        impl Method for SetMediaText {
            const NAME: &'static str = "CSS.setMediaText";
            type ReturnObject = SetMediaTextReturnObject;
        }
        impl Method for SetContainerQueryText {
            const NAME: &'static str = "CSS.setContainerQueryText";
            type ReturnObject = SetContainerQueryTextReturnObject;
        }
        impl Method for SetRuleSelector {
            const NAME: &'static str = "CSS.setRuleSelector";
            type ReturnObject = SetRuleSelectorReturnObject;
        }
        impl Method for SetStyleSheetText {
            const NAME: &'static str = "CSS.setStyleSheetText";
            type ReturnObject = SetStyleSheetTextReturnObject;
        }
        impl Method for SetStyleTexts {
            const NAME: &'static str = "CSS.setStyleTexts";
            type ReturnObject = SetStyleTextsReturnObject;
        }
        impl Method for StartRuleUsageTracking {
            const NAME: &'static str = "CSS.startRuleUsageTracking";
            type ReturnObject = StartRuleUsageTrackingReturnObject;
        }
        impl Method for StopRuleUsageTracking {
            const NAME: &'static str = "CSS.stopRuleUsageTracking";
            type ReturnObject = StopRuleUsageTrackingReturnObject;
        }
        impl Method for TakeCoverageDelta {
            const NAME: &'static str = "CSS.takeCoverageDelta";
            type ReturnObject = TakeCoverageDeltaReturnObject;
        }
        impl Method for SetLocalFontsEnabled {
            const NAME: &'static str = "CSS.setLocalFontsEnabled";
            type ReturnObject = SetLocalFontsEnabledReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FontsUpdatedEvent {
                pub params: FontsUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FontsUpdatedEventParams {
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "font")]
                pub font: Option<super::FontFace>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct MediaQueryResultChangedEvent(pub Option<serde_json::Value>);
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct StyleSheetAddedEvent {
                pub params: StyleSheetAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct StyleSheetAddedEventParams {
                #[serde(rename = "header")]
                pub header: super::CSSStyleSheetHeader,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct StyleSheetChangedEvent {
                pub params: StyleSheetChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct StyleSheetChangedEventParams {
                #[serde(rename = "styleSheetId")]
                pub style_sheet_id: super::StyleSheetId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct StyleSheetRemovedEvent {
                pub params: StyleSheetRemovedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct StyleSheetRemovedEventParams {
                #[serde(rename = "styleSheetId")]
                pub style_sheet_id: super::StyleSheetId,
            }
        }
    }
    pub mod CacheStorage {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type CacheId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum CachedResponseType {
            #[serde(rename = "basic")]
            Basic,
            #[serde(rename = "cors")]
            Cors,
            #[serde(rename = "default")]
            Default,
            #[serde(rename = "error")]
            Error,
            #[serde(rename = "opaqueResponse")]
            OpaqueResponse,
            #[serde(rename = "opaqueRedirect")]
            OpaqueRedirect,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DataEntry {
            #[serde(default)]
            #[serde(rename = "requestURL")]
            pub request_url: String,
            #[serde(default)]
            #[serde(rename = "requestMethod")]
            pub request_method: String,
            #[serde(rename = "requestHeaders")]
            pub request_headers: Vec<Header>,
            #[serde(default)]
            #[serde(rename = "responseTime")]
            pub response_time: JsFloat,
            #[serde(default)]
            #[serde(rename = "responseStatus")]
            pub response_status: JsUInt,
            #[serde(default)]
            #[serde(rename = "responseStatusText")]
            pub response_status_text: String,
            #[serde(rename = "responseType")]
            pub response_Type: CachedResponseType,
            #[serde(rename = "responseHeaders")]
            pub response_headers: Vec<Header>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Cache {
            #[serde(rename = "cacheId")]
            pub cache_id: CacheId,
            #[serde(default)]
            #[serde(rename = "securityOrigin")]
            pub security_origin: String,
            #[serde(default)]
            #[serde(rename = "cacheName")]
            pub cache_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Header {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CachedResponse {
            #[serde(default)]
            #[serde(rename = "body")]
            pub body: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DeleteCache {
            #[serde(rename = "cacheId")]
            pub cache_id: CacheId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DeleteEntry {
            #[serde(rename = "cacheId")]
            pub cache_id: CacheId,
            #[serde(default)]
            #[serde(rename = "request")]
            pub request: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestCacheNames {
            #[serde(default)]
            #[serde(rename = "securityOrigin")]
            pub security_origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestCachedResponse {
            #[serde(rename = "cacheId")]
            pub cache_id: CacheId,
            #[serde(default)]
            #[serde(rename = "requestURL")]
            pub request_url: String,
            #[serde(rename = "requestHeaders")]
            pub request_headers: Vec<Header>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestEntries {
            #[serde(rename = "cacheId")]
            pub cache_id: CacheId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "skipCount")]
            pub skip_count: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "pageSize")]
            pub page_size: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "pathFilter")]
            pub path_filter: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteCacheReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteEntryReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestCacheNamesReturnObject {
            #[serde(rename = "caches")]
            pub caches: Vec<Cache>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestCachedResponseReturnObject {
            #[serde(rename = "response")]
            pub response: CachedResponse,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestEntriesReturnObject {
            #[serde(rename = "cacheDataEntries")]
            pub cache_data_entries: Vec<DataEntry>,
            #[serde(default)]
            #[serde(rename = "returnCount")]
            pub return_count: JsFloat,
        }
        impl Method for DeleteCache {
            const NAME: &'static str = "CacheStorage.deleteCache";
            type ReturnObject = DeleteCacheReturnObject;
        }
        impl Method for DeleteEntry {
            const NAME: &'static str = "CacheStorage.deleteEntry";
            type ReturnObject = DeleteEntryReturnObject;
        }
        impl Method for RequestCacheNames {
            const NAME: &'static str = "CacheStorage.requestCacheNames";
            type ReturnObject = RequestCacheNamesReturnObject;
        }
        impl Method for RequestCachedResponse {
            const NAME: &'static str = "CacheStorage.requestCachedResponse";
            type ReturnObject = RequestCachedResponseReturnObject;
        }
        impl Method for RequestEntries {
            const NAME: &'static str = "CacheStorage.requestEntries";
            type ReturnObject = RequestEntriesReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod Cast {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Sink {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "id")]
            pub id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "session")]
            pub session: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Enable {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "presentationUrl")]
            pub presentation_url: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetSinkToUse {
            #[serde(default)]
            #[serde(rename = "sinkName")]
            pub sink_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StartTabMirroring {
            #[serde(default)]
            #[serde(rename = "sinkName")]
            pub sink_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StopCasting {
            #[serde(default)]
            #[serde(rename = "sinkName")]
            pub sink_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetSinkToUseReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartTabMirroringReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopCastingReturnObject {}
        impl Method for Enable {
            const NAME: &'static str = "Cast.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Cast.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for SetSinkToUse {
            const NAME: &'static str = "Cast.setSinkToUse";
            type ReturnObject = SetSinkToUseReturnObject;
        }
        impl Method for StartTabMirroring {
            const NAME: &'static str = "Cast.startTabMirroring";
            type ReturnObject = StartTabMirroringReturnObject;
        }
        impl Method for StopCasting {
            const NAME: &'static str = "Cast.stopCasting";
            type ReturnObject = StopCastingReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SinksUpdatedEvent {
                pub params: SinksUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SinksUpdatedEventParams {
                #[serde(rename = "sinks")]
                pub sinks: Vec<super::Sink>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct IssueUpdatedEvent {
                pub params: IssueUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct IssueUpdatedEventParams {
                #[serde(default)]
                #[serde(rename = "issueMessage")]
                pub issue_message: String,
            }
        }
    }
    pub mod DOM {
        use super::types::*;
        use super::Page;
        use super::Runtime;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type NodeId = JsUInt;
        pub type BackendNodeId = JsUInt;
        pub type Quad = Vec<JsFloat>;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum PseudoType {
            #[serde(rename = "first-line")]
            FirstLine,
            #[serde(rename = "first-letter")]
            FirstLetter,
            #[serde(rename = "before")]
            Before,
            #[serde(rename = "after")]
            After,
            #[serde(rename = "marker")]
            Marker,
            #[serde(rename = "backdrop")]
            Backdrop,
            #[serde(rename = "selection")]
            Selection,
            #[serde(rename = "target-text")]
            TargetText,
            #[serde(rename = "spelling-error")]
            SpellingError,
            #[serde(rename = "grammar-error")]
            GrammarError,
            #[serde(rename = "highlight")]
            Highlight,
            #[serde(rename = "first-line-inherited")]
            FirstLineInherited,
            #[serde(rename = "scrollbar")]
            Scrollbar,
            #[serde(rename = "scrollbar-thumb")]
            ScrollbarThumb,
            #[serde(rename = "scrollbar-button")]
            ScrollbarButton,
            #[serde(rename = "scrollbar-track")]
            ScrollbarTrack,
            #[serde(rename = "scrollbar-track-piece")]
            ScrollbarTrackPiece,
            #[serde(rename = "scrollbar-corner")]
            ScrollbarCorner,
            #[serde(rename = "resizer")]
            Resizer,
            #[serde(rename = "input-list-button")]
            InputListButton,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ShadowRootType {
            #[serde(rename = "user-agent")]
            UserAgent,
            #[serde(rename = "open")]
            Open,
            #[serde(rename = "closed")]
            Closed,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum CompatibilityMode {
            #[serde(rename = "QuirksMode")]
            QuirksMode,
            #[serde(rename = "LimitedQuirksMode")]
            LimitedQuirksMode,
            #[serde(rename = "NoQuirksMode")]
            NoQuirksMode,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct BackendNode {
            #[serde(default)]
            #[serde(rename = "nodeType")]
            pub node_type: JsUInt,
            #[serde(default)]
            #[serde(rename = "nodeName")]
            pub node_name: String,
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: BackendNodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Node {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "parentId")]
            pub parent_id: Option<NodeId>,
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: BackendNodeId,
            #[serde(default)]
            #[serde(rename = "nodeType")]
            pub node_type: JsUInt,
            #[serde(default)]
            #[serde(rename = "nodeName")]
            pub node_name: String,
            #[serde(default)]
            #[serde(rename = "localName")]
            pub local_name: String,
            #[serde(default)]
            #[serde(rename = "nodeValue")]
            pub node_value: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "childNodeCount")]
            pub child_node_count: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "children")]
            pub children: Option<Vec<Node>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "attributes")]
            pub attributes: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "documentURL")]
            pub document_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "baseURL")]
            pub base_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "publicId")]
            pub public_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "systemId")]
            pub system_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "internalSubset")]
            pub internal_subset: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "xmlVersion")]
            pub xml_version: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "pseudoType")]
            pub pseudo_Type: Option<PseudoType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "shadowRootType")]
            pub shadow_root_Type: Option<ShadowRootType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "frameId")]
            pub frame_id: Option<Page::FrameId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "contentDocument")]
            pub content_document: Option<Box<Node>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "shadowRoots")]
            pub shadow_roots: Option<Vec<Node>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "templateContent")]
            pub template_content: Option<Box<Node>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "pseudoElements")]
            pub pseudo_elements: Option<Vec<Node>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "importedDocument")]
            pub imported_document: Option<Box<Node>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "distributedNodes")]
            pub distributed_nodes: Option<Vec<BackendNode>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "isSVG")]
            pub is_svg: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "compatibilityMode")]
            pub compatibility_mode: Option<CompatibilityMode>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RGBA {
            #[serde(default)]
            #[serde(rename = "r")]
            pub r: JsUInt,
            #[serde(default)]
            #[serde(rename = "g")]
            pub g: JsUInt,
            #[serde(default)]
            #[serde(rename = "b")]
            pub b: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "a")]
            pub a: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct BoxModel {
            #[serde(rename = "content")]
            pub content: Quad,
            #[serde(rename = "padding")]
            pub padding: Quad,
            #[serde(rename = "border")]
            pub border: Quad,
            #[serde(rename = "margin")]
            pub margin: Quad,
            #[serde(default)]
            #[serde(rename = "width")]
            pub width: JsUInt,
            #[serde(default)]
            #[serde(rename = "height")]
            pub height: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "shapeOutside")]
            pub shape_outside: Option<ShapeOutsideInfo>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ShapeOutsideInfo {
            #[serde(rename = "bounds")]
            pub bounds: Quad,
            #[serde(default)]
            #[serde(rename = "shape")]
            pub shape: Vec<Json>,
            #[serde(default)]
            #[serde(rename = "marginShape")]
            pub margin_shape: Vec<Json>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Rect {
            #[serde(default)]
            #[serde(rename = "x")]
            pub x: JsFloat,
            #[serde(default)]
            #[serde(rename = "y")]
            pub y: JsFloat,
            #[serde(default)]
            #[serde(rename = "width")]
            pub width: JsFloat,
            #[serde(default)]
            #[serde(rename = "height")]
            pub height: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CSSComputedStyleProperty {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CollectClassNamesFromSubtree {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CopyTo {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
            #[serde(rename = "targetNodeId")]
            pub target_node_id: NodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "insertBeforeNodeId")]
            pub insert_before_node_id: Option<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DescribeNode {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: Option<BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "objectId")]
            pub object_id: Option<Runtime::RemoteObjectId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "depth")]
            pub depth: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "pierce")]
            pub pierce: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ScrollIntoViewIfNeeded {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: Option<BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "objectId")]
            pub object_id: Option<Runtime::RemoteObjectId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "rect")]
            pub rect: Option<Rect>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DiscardSearchResults {
            #[serde(default)]
            #[serde(rename = "searchId")]
            pub search_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Focus {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: Option<BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "objectId")]
            pub object_id: Option<Runtime::RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetAttributes {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetBoxModel {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: Option<BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "objectId")]
            pub object_id: Option<Runtime::RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetContentQuads {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: Option<BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "objectId")]
            pub object_id: Option<Runtime::RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetDocument {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "depth")]
            pub depth: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "pierce")]
            pub pierce: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetFlattenedDocument {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "depth")]
            pub depth: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "pierce")]
            pub pierce: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetNodesForSubtreeByStyle {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
            #[serde(rename = "computedStyles")]
            pub computed_styles: Vec<CSSComputedStyleProperty>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "pierce")]
            pub pierce: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetNodeForLocation {
            #[serde(default)]
            #[serde(rename = "x")]
            pub x: JsUInt,
            #[serde(default)]
            #[serde(rename = "y")]
            pub y: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "includeUserAgentShadowDOM")]
            pub include_user_agent_shadow_dom: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "ignorePointerEventsNone")]
            pub ignore_pointer_events_none: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetOuterHTML {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: Option<BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "objectId")]
            pub object_id: Option<Runtime::RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetRelayoutBoundary {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetSearchResults {
            #[serde(default)]
            #[serde(rename = "searchId")]
            pub search_id: String,
            #[serde(default)]
            #[serde(rename = "fromIndex")]
            pub from_index: JsUInt,
            #[serde(default)]
            #[serde(rename = "toIndex")]
            pub to_index: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HideHighlight(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightNode(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightRect(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct MarkUndoableState(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct MoveTo {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
            #[serde(rename = "targetNodeId")]
            pub target_node_id: NodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "insertBeforeNodeId")]
            pub insert_before_node_id: Option<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PerformSearch {
            #[serde(default)]
            #[serde(rename = "query")]
            pub query: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "includeUserAgentShadowDOM")]
            pub include_user_agent_shadow_dom: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PushNodeByPathToFrontend {
            #[serde(default)]
            #[serde(rename = "path")]
            pub path: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PushNodesByBackendIdsToFrontend {
            #[serde(rename = "backendNodeIds")]
            pub backend_node_ids: Vec<BackendNodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct QuerySelector {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
            #[serde(default)]
            #[serde(rename = "selector")]
            pub selector: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct QuerySelectorAll {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
            #[serde(default)]
            #[serde(rename = "selector")]
            pub selector: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Redo(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RemoveAttribute {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RemoveNode {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestChildNodes {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "depth")]
            pub depth: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "pierce")]
            pub pierce: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestNode {
            #[serde(rename = "objectId")]
            pub object_id: Runtime::RemoteObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ResolveNode {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "objectGroup")]
            pub object_group: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "executionContextId")]
            pub execution_context_id: Option<Runtime::ExecutionContextId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetAttributeValue {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetAttributesAsText {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetFileInputFiles {
            #[serde(default)]
            #[serde(rename = "files")]
            pub files: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: Option<BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "objectId")]
            pub object_id: Option<Runtime::RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetNodeStackTracesEnabled {
            #[serde(default)]
            #[serde(rename = "enable")]
            pub enable: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetNodeStackTraces {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetFileInfo {
            #[serde(rename = "objectId")]
            pub object_id: Runtime::RemoteObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetInspectedNode {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetNodeName {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetNodeValue {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetOuterHTML {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
            #[serde(default)]
            #[serde(rename = "outerHTML")]
            pub outer_html: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Undo(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetFrameOwner {
            #[serde(rename = "frameId")]
            pub frame_id: Page::FrameId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetContainerForNode {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "containerName")]
            pub container_name: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetQueryingDescendantsForContainer {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CollectClassNamesFromSubtreeReturnObject {
            #[serde(rename = "classNames")]
            pub class_names: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CopyToReturnObject {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DescribeNodeReturnObject {
            #[serde(rename = "node")]
            pub node: Node,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ScrollIntoViewIfNeededReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DiscardSearchResultsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FocusReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetAttributesReturnObject {
            #[serde(rename = "attributes")]
            pub attributes: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetBoxModelReturnObject {
            #[serde(rename = "model")]
            pub model: BoxModel,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetContentQuadsReturnObject {
            #[serde(rename = "quads")]
            pub quads: Vec<Quad>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetDocumentReturnObject {
            #[serde(rename = "root")]
            pub root: Node,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetFlattenedDocumentReturnObject {
            #[serde(rename = "nodes")]
            pub nodes: Vec<Node>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetNodesForSubtreeByStyleReturnObject {
            #[serde(rename = "nodeIds")]
            pub node_ids: Vec<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetNodeForLocationReturnObject {
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: BackendNodeId,
            #[serde(rename = "frameId")]
            pub frame_id: Page::FrameId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetOuterHTMLReturnObject {
            #[serde(default)]
            #[serde(rename = "outerHTML")]
            pub outer_html: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetRelayoutBoundaryReturnObject {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetSearchResultsReturnObject {
            #[serde(rename = "nodeIds")]
            pub node_ids: Vec<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HideHighlightReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightNodeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightRectReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct MarkUndoableStateReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct MoveToReturnObject {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PerformSearchReturnObject {
            #[serde(default)]
            #[serde(rename = "searchId")]
            pub search_id: String,
            #[serde(default)]
            #[serde(rename = "resultCount")]
            pub result_count: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PushNodeByPathToFrontendReturnObject {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PushNodesByBackendIdsToFrontendReturnObject {
            #[serde(rename = "nodeIds")]
            pub node_ids: Vec<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct QuerySelectorReturnObject {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct QuerySelectorAllReturnObject {
            #[serde(rename = "nodeIds")]
            pub node_ids: Vec<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RedoReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveAttributeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveNodeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestChildNodesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestNodeReturnObject {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ResolveNodeReturnObject {
            #[serde(rename = "object")]
            pub object: Runtime::RemoteObject,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAttributeValueReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAttributesAsTextReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetFileInputFilesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetNodeStackTracesEnabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetNodeStackTracesReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "creation")]
            pub creation: Option<Runtime::StackTrace>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetFileInfoReturnObject {
            #[serde(default)]
            #[serde(rename = "path")]
            pub path: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInspectedNodeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetNodeNameReturnObject {
            #[serde(rename = "nodeId")]
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetNodeValueReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetOuterHTMLReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UndoReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetFrameOwnerReturnObject {
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: BackendNodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetContainerForNodeReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetQueryingDescendantsForContainerReturnObject {
            #[serde(rename = "nodeIds")]
            pub node_ids: Vec<NodeId>,
        }
        impl Method for CollectClassNamesFromSubtree {
            const NAME: &'static str = "DOM.collectClassNamesFromSubtree";
            type ReturnObject = CollectClassNamesFromSubtreeReturnObject;
        }
        impl Method for CopyTo {
            const NAME: &'static str = "DOM.copyTo";
            type ReturnObject = CopyToReturnObject;
        }
        impl Method for DescribeNode {
            const NAME: &'static str = "DOM.describeNode";
            type ReturnObject = DescribeNodeReturnObject;
        }
        impl Method for ScrollIntoViewIfNeeded {
            const NAME: &'static str = "DOM.scrollIntoViewIfNeeded";
            type ReturnObject = ScrollIntoViewIfNeededReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "DOM.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for DiscardSearchResults {
            const NAME: &'static str = "DOM.discardSearchResults";
            type ReturnObject = DiscardSearchResultsReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "DOM.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for Focus {
            const NAME: &'static str = "DOM.focus";
            type ReturnObject = FocusReturnObject;
        }
        impl Method for GetAttributes {
            const NAME: &'static str = "DOM.getAttributes";
            type ReturnObject = GetAttributesReturnObject;
        }
        impl Method for GetBoxModel {
            const NAME: &'static str = "DOM.getBoxModel";
            type ReturnObject = GetBoxModelReturnObject;
        }
        impl Method for GetContentQuads {
            const NAME: &'static str = "DOM.getContentQuads";
            type ReturnObject = GetContentQuadsReturnObject;
        }
        impl Method for GetDocument {
            const NAME: &'static str = "DOM.getDocument";
            type ReturnObject = GetDocumentReturnObject;
        }
        impl Method for GetFlattenedDocument {
            const NAME: &'static str = "DOM.getFlattenedDocument";
            type ReturnObject = GetFlattenedDocumentReturnObject;
        }
        impl Method for GetNodesForSubtreeByStyle {
            const NAME: &'static str = "DOM.getNodesForSubtreeByStyle";
            type ReturnObject = GetNodesForSubtreeByStyleReturnObject;
        }
        impl Method for GetNodeForLocation {
            const NAME: &'static str = "DOM.getNodeForLocation";
            type ReturnObject = GetNodeForLocationReturnObject;
        }
        impl Method for GetOuterHTML {
            const NAME: &'static str = "DOM.getOuterHTML";
            type ReturnObject = GetOuterHTMLReturnObject;
        }
        impl Method for GetRelayoutBoundary {
            const NAME: &'static str = "DOM.getRelayoutBoundary";
            type ReturnObject = GetRelayoutBoundaryReturnObject;
        }
        impl Method for GetSearchResults {
            const NAME: &'static str = "DOM.getSearchResults";
            type ReturnObject = GetSearchResultsReturnObject;
        }
        impl Method for HideHighlight {
            const NAME: &'static str = "DOM.hideHighlight";
            type ReturnObject = HideHighlightReturnObject;
        }
        impl Method for HighlightNode {
            const NAME: &'static str = "DOM.highlightNode";
            type ReturnObject = HighlightNodeReturnObject;
        }
        impl Method for HighlightRect {
            const NAME: &'static str = "DOM.highlightRect";
            type ReturnObject = HighlightRectReturnObject;
        }
        impl Method for MarkUndoableState {
            const NAME: &'static str = "DOM.markUndoableState";
            type ReturnObject = MarkUndoableStateReturnObject;
        }
        impl Method for MoveTo {
            const NAME: &'static str = "DOM.moveTo";
            type ReturnObject = MoveToReturnObject;
        }
        impl Method for PerformSearch {
            const NAME: &'static str = "DOM.performSearch";
            type ReturnObject = PerformSearchReturnObject;
        }
        impl Method for PushNodeByPathToFrontend {
            const NAME: &'static str = "DOM.pushNodeByPathToFrontend";
            type ReturnObject = PushNodeByPathToFrontendReturnObject;
        }
        impl Method for PushNodesByBackendIdsToFrontend {
            const NAME: &'static str = "DOM.pushNodesByBackendIdsToFrontend";
            type ReturnObject = PushNodesByBackendIdsToFrontendReturnObject;
        }
        impl Method for QuerySelector {
            const NAME: &'static str = "DOM.querySelector";
            type ReturnObject = QuerySelectorReturnObject;
        }
        impl Method for QuerySelectorAll {
            const NAME: &'static str = "DOM.querySelectorAll";
            type ReturnObject = QuerySelectorAllReturnObject;
        }
        impl Method for Redo {
            const NAME: &'static str = "DOM.redo";
            type ReturnObject = RedoReturnObject;
        }
        impl Method for RemoveAttribute {
            const NAME: &'static str = "DOM.removeAttribute";
            type ReturnObject = RemoveAttributeReturnObject;
        }
        impl Method for RemoveNode {
            const NAME: &'static str = "DOM.removeNode";
            type ReturnObject = RemoveNodeReturnObject;
        }
        impl Method for RequestChildNodes {
            const NAME: &'static str = "DOM.requestChildNodes";
            type ReturnObject = RequestChildNodesReturnObject;
        }
        impl Method for RequestNode {
            const NAME: &'static str = "DOM.requestNode";
            type ReturnObject = RequestNodeReturnObject;
        }
        impl Method for ResolveNode {
            const NAME: &'static str = "DOM.resolveNode";
            type ReturnObject = ResolveNodeReturnObject;
        }
        impl Method for SetAttributeValue {
            const NAME: &'static str = "DOM.setAttributeValue";
            type ReturnObject = SetAttributeValueReturnObject;
        }
        impl Method for SetAttributesAsText {
            const NAME: &'static str = "DOM.setAttributesAsText";
            type ReturnObject = SetAttributesAsTextReturnObject;
        }
        impl Method for SetFileInputFiles {
            const NAME: &'static str = "DOM.setFileInputFiles";
            type ReturnObject = SetFileInputFilesReturnObject;
        }
        impl Method for SetNodeStackTracesEnabled {
            const NAME: &'static str = "DOM.setNodeStackTracesEnabled";
            type ReturnObject = SetNodeStackTracesEnabledReturnObject;
        }
        impl Method for GetNodeStackTraces {
            const NAME: &'static str = "DOM.getNodeStackTraces";
            type ReturnObject = GetNodeStackTracesReturnObject;
        }
        impl Method for GetFileInfo {
            const NAME: &'static str = "DOM.getFileInfo";
            type ReturnObject = GetFileInfoReturnObject;
        }
        impl Method for SetInspectedNode {
            const NAME: &'static str = "DOM.setInspectedNode";
            type ReturnObject = SetInspectedNodeReturnObject;
        }
        impl Method for SetNodeName {
            const NAME: &'static str = "DOM.setNodeName";
            type ReturnObject = SetNodeNameReturnObject;
        }
        impl Method for SetNodeValue {
            const NAME: &'static str = "DOM.setNodeValue";
            type ReturnObject = SetNodeValueReturnObject;
        }
        impl Method for SetOuterHTML {
            const NAME: &'static str = "DOM.setOuterHTML";
            type ReturnObject = SetOuterHTMLReturnObject;
        }
        impl Method for Undo {
            const NAME: &'static str = "DOM.undo";
            type ReturnObject = UndoReturnObject;
        }
        impl Method for GetFrameOwner {
            const NAME: &'static str = "DOM.getFrameOwner";
            type ReturnObject = GetFrameOwnerReturnObject;
        }
        impl Method for GetContainerForNode {
            const NAME: &'static str = "DOM.getContainerForNode";
            type ReturnObject = GetContainerForNodeReturnObject;
        }
        impl Method for GetQueryingDescendantsForContainer {
            const NAME: &'static str = "DOM.getQueryingDescendantsForContainer";
            type ReturnObject = GetQueryingDescendantsForContainerReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AttributeModifiedEvent {
                pub params: AttributeModifiedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AttributeModifiedEventParams {
                #[serde(rename = "nodeId")]
                pub node_id: super::NodeId,
                #[serde(default)]
                #[serde(rename = "name")]
                pub name: String,
                #[serde(default)]
                #[serde(rename = "value")]
                pub value: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AttributeRemovedEvent {
                pub params: AttributeRemovedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AttributeRemovedEventParams {
                #[serde(rename = "nodeId")]
                pub node_id: super::NodeId,
                #[serde(default)]
                #[serde(rename = "name")]
                pub name: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct CharacterDataModifiedEvent {
                pub params: CharacterDataModifiedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct CharacterDataModifiedEventParams {
                #[serde(rename = "nodeId")]
                pub node_id: super::NodeId,
                #[serde(default)]
                #[serde(rename = "characterData")]
                pub character_data: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ChildNodeCountUpdatedEvent {
                pub params: ChildNodeCountUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ChildNodeCountUpdatedEventParams {
                #[serde(rename = "nodeId")]
                pub node_id: super::NodeId,
                #[serde(default)]
                #[serde(rename = "childNodeCount")]
                pub child_node_count: JsUInt,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ChildNodeInsertedEvent {
                pub params: ChildNodeInsertedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ChildNodeInsertedEventParams {
                #[serde(rename = "parentNodeId")]
                pub parent_node_id: super::NodeId,
                #[serde(rename = "previousNodeId")]
                pub previous_node_id: super::NodeId,
                #[serde(rename = "node")]
                pub node: super::Node,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ChildNodeRemovedEvent {
                pub params: ChildNodeRemovedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ChildNodeRemovedEventParams {
                #[serde(rename = "parentNodeId")]
                pub parent_node_id: super::NodeId,
                #[serde(rename = "nodeId")]
                pub node_id: super::NodeId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DistributedNodesUpdatedEvent {
                pub params: DistributedNodesUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DistributedNodesUpdatedEventParams {
                #[serde(rename = "insertionPointId")]
                pub insertion_point_id: super::NodeId,
                #[serde(rename = "distributedNodes")]
                pub distributed_nodes: Vec<super::BackendNode>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DocumentUpdatedEvent(pub Option<serde_json::Value>);
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct InlineStyleInvalidatedEvent {
                pub params: InlineStyleInvalidatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct InlineStyleInvalidatedEventParams {
                #[serde(rename = "nodeIds")]
                pub node_ids: Vec<super::NodeId>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PseudoElementAddedEvent {
                pub params: PseudoElementAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PseudoElementAddedEventParams {
                #[serde(rename = "parentId")]
                pub parent_id: super::NodeId,
                #[serde(rename = "pseudoElement")]
                pub pseudo_element: super::Node,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PseudoElementRemovedEvent {
                pub params: PseudoElementRemovedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PseudoElementRemovedEventParams {
                #[serde(rename = "parentId")]
                pub parent_id: super::NodeId,
                #[serde(rename = "pseudoElementId")]
                pub pseudo_element_id: super::NodeId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SetChildNodesEvent {
                pub params: SetChildNodesEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SetChildNodesEventParams {
                #[serde(rename = "parentId")]
                pub parent_id: super::NodeId,
                #[serde(rename = "nodes")]
                pub nodes: Vec<super::Node>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ShadowRootPoppedEvent {
                pub params: ShadowRootPoppedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ShadowRootPoppedEventParams {
                #[serde(rename = "hostId")]
                pub host_id: super::NodeId,
                #[serde(rename = "rootId")]
                pub root_id: super::NodeId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ShadowRootPushedEvent {
                pub params: ShadowRootPushedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ShadowRootPushedEventParams {
                #[serde(rename = "hostId")]
                pub host_id: super::NodeId,
                #[serde(rename = "root")]
                pub root: super::Node,
            }
        }
    }
    pub mod DOMDebugger {
        use super::types::*;
        use super::Debugger;
        use super::Runtime;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum DOMBreakpointType {
            #[serde(rename = "subtree-modified")]
            SubtreeModified,
            #[serde(rename = "attribute-modified")]
            AttributeModified,
            #[serde(rename = "node-removed")]
            NodeRemoved,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum CSPViolationType {
            #[serde(rename = "trustedtype-sink-violation")]
            TrustedtypeSinkViolation,
            #[serde(rename = "trustedtype-policy-violation")]
            TrustedtypePolicyViolation,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct EventListener {
            #[serde(default)]
            #[serde(rename = "type")]
            pub Type: String,
            #[serde(default)]
            #[serde(rename = "useCapture")]
            pub use_capture: bool,
            #[serde(default)]
            #[serde(rename = "passive")]
            pub passive: bool,
            #[serde(default)]
            #[serde(rename = "once")]
            pub once: bool,
            #[serde(rename = "scriptId")]
            pub script_id: Runtime::ScriptId,
            #[serde(default)]
            #[serde(rename = "lineNumber")]
            pub line_number: JsUInt,
            #[serde(default)]
            #[serde(rename = "columnNumber")]
            pub column_number: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "handler")]
            pub handler: Option<Runtime::RemoteObject>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "originalHandler")]
            pub original_handler: Option<Runtime::RemoteObject>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetEventListeners {
            #[serde(rename = "objectId")]
            pub object_id: Runtime::RemoteObjectId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "depth")]
            pub depth: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "pierce")]
            pub pierce: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RemoveDOMBreakpoint {
            #[serde(rename = "nodeId")]
            pub node_id: DOM::NodeId,
            #[serde(rename = "type")]
            pub Type: DOMBreakpointType,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RemoveEventListenerBreakpoint {
            #[serde(default)]
            #[serde(rename = "eventName")]
            pub event_name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "targetName")]
            pub target_name: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RemoveInstrumentationBreakpoint {
            #[serde(default)]
            #[serde(rename = "eventName")]
            pub event_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RemoveXHRBreakpoint {
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetBreakOnCSPViolation {
            #[serde(rename = "violationTypes")]
            pub violation_Types: Vec<CSPViolationType>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetDOMBreakpoint {
            #[serde(rename = "nodeId")]
            pub node_id: DOM::NodeId,
            #[serde(rename = "type")]
            pub Type: DOMBreakpointType,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetEventListenerBreakpoint {
            #[serde(default)]
            #[serde(rename = "eventName")]
            pub event_name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "targetName")]
            pub target_name: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetInstrumentationBreakpoint {
            #[serde(default)]
            #[serde(rename = "eventName")]
            pub event_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetXHRBreakpoint {
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetEventListenersReturnObject {
            #[serde(rename = "listeners")]
            pub listeners: Vec<EventListener>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveDOMBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveEventListenerBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveInstrumentationBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveXHRBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBreakOnCSPViolationReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDOMBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetEventListenerBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInstrumentationBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetXHRBreakpointReturnObject {}
        impl Method for GetEventListeners {
            const NAME: &'static str = "DOMDebugger.getEventListeners";
            type ReturnObject = GetEventListenersReturnObject;
        }
        impl Method for RemoveDOMBreakpoint {
            const NAME: &'static str = "DOMDebugger.removeDOMBreakpoint";
            type ReturnObject = RemoveDOMBreakpointReturnObject;
        }
        impl Method for RemoveEventListenerBreakpoint {
            const NAME: &'static str = "DOMDebugger.removeEventListenerBreakpoint";
            type ReturnObject = RemoveEventListenerBreakpointReturnObject;
        }
        impl Method for RemoveInstrumentationBreakpoint {
            const NAME: &'static str = "DOMDebugger.removeInstrumentationBreakpoint";
            type ReturnObject = RemoveInstrumentationBreakpointReturnObject;
        }
        impl Method for RemoveXHRBreakpoint {
            const NAME: &'static str = "DOMDebugger.removeXHRBreakpoint";
            type ReturnObject = RemoveXHRBreakpointReturnObject;
        }
        impl Method for SetBreakOnCSPViolation {
            const NAME: &'static str = "DOMDebugger.setBreakOnCSPViolation";
            type ReturnObject = SetBreakOnCSPViolationReturnObject;
        }
        impl Method for SetDOMBreakpoint {
            const NAME: &'static str = "DOMDebugger.setDOMBreakpoint";
            type ReturnObject = SetDOMBreakpointReturnObject;
        }
        impl Method for SetEventListenerBreakpoint {
            const NAME: &'static str = "DOMDebugger.setEventListenerBreakpoint";
            type ReturnObject = SetEventListenerBreakpointReturnObject;
        }
        impl Method for SetInstrumentationBreakpoint {
            const NAME: &'static str = "DOMDebugger.setInstrumentationBreakpoint";
            type ReturnObject = SetInstrumentationBreakpointReturnObject;
        }
        impl Method for SetXHRBreakpoint {
            const NAME: &'static str = "DOMDebugger.setXHRBreakpoint";
            type ReturnObject = SetXHRBreakpointReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod EventBreakpoints {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetInstrumentationBreakpoint {
            #[serde(default)]
            #[serde(rename = "eventName")]
            pub event_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RemoveInstrumentationBreakpoint {
            #[serde(default)]
            #[serde(rename = "eventName")]
            pub event_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInstrumentationBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveInstrumentationBreakpointReturnObject {}
        impl Method for SetInstrumentationBreakpoint {
            const NAME: &'static str = "EventBreakpoints.setInstrumentationBreakpoint";
            type ReturnObject = SetInstrumentationBreakpointReturnObject;
        }
        impl Method for RemoveInstrumentationBreakpoint {
            const NAME: &'static str = "EventBreakpoints.removeInstrumentationBreakpoint";
            type ReturnObject = RemoveInstrumentationBreakpointReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod DOMSnapshot {
        use super::types::*;
        use super::DOMDebugger;
        use super::Page;
        use super::CSS;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type StringIndex = JsUInt;
        pub type ArrayOfStrings = Vec<StringIndex>;
        pub type Rectangle = Vec<JsFloat>;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DOMNode {
            #[serde(default)]
            #[serde(rename = "nodeType")]
            pub node_type: JsUInt,
            #[serde(default)]
            #[serde(rename = "nodeName")]
            pub node_name: String,
            #[serde(default)]
            #[serde(rename = "nodeValue")]
            pub node_value: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "textValue")]
            pub text_value: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "inputValue")]
            pub input_value: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "inputChecked")]
            pub input_checked: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "optionSelected")]
            pub option_selected: Option<bool>,
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: DOM::BackendNodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "childNodeIndexes")]
            pub child_node_indexes: Option<Vec<JsUInt>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "attributes")]
            pub attributes: Option<Vec<NameValue>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "pseudoElementIndexes")]
            pub pseudo_element_indexes: Option<Vec<JsUInt>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "layoutNodeIndex")]
            pub layout_node_index: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "documentURL")]
            pub document_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "baseURL")]
            pub base_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "contentLanguage")]
            pub content_language: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "documentEncoding")]
            pub document_encoding: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "publicId")]
            pub public_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "systemId")]
            pub system_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "frameId")]
            pub frame_id: Option<Page::FrameId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "contentDocumentIndex")]
            pub content_document_index: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "pseudoType")]
            pub pseudo_Type: Option<DOM::PseudoType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "shadowRootType")]
            pub shadow_root_Type: Option<DOM::ShadowRootType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "isClickable")]
            pub is_clickable: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "eventListeners")]
            pub event_listeners: Option<Vec<DOMDebugger::EventListener>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "currentSourceURL")]
            pub current_source_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "originURL")]
            pub origin_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "scrollOffsetX")]
            pub scroll_offset_x: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "scrollOffsetY")]
            pub scroll_offset_y: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct InlineTextBox {
            #[serde(rename = "boundingBox")]
            pub bounding_box: DOM::Rect,
            #[serde(default)]
            #[serde(rename = "startCharacterIndex")]
            pub start_character_index: JsUInt,
            #[serde(default)]
            #[serde(rename = "numCharacters")]
            pub num_characters: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct LayoutTreeNode {
            #[serde(default)]
            #[serde(rename = "domNodeIndex")]
            pub dom_node_index: JsUInt,
            #[serde(rename = "boundingBox")]
            pub bounding_box: DOM::Rect,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "layoutText")]
            pub layout_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "inlineTextNodes")]
            pub inline_text_nodes: Option<Vec<InlineTextBox>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "styleIndex")]
            pub style_index: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "paintOrder")]
            pub paint_order: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "isStackingContext")]
            pub is_stacking_context: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ComputedStyle {
            #[serde(rename = "properties")]
            pub properties: Vec<NameValue>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct NameValue {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RareStringData {
            #[serde(default)]
            #[serde(rename = "index")]
            pub index: Vec<JsUInt>,
            #[serde(rename = "value")]
            pub value: Vec<StringIndex>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RareBooleanData {
            #[serde(default)]
            #[serde(rename = "index")]
            pub index: Vec<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RareIntegerData {
            #[serde(default)]
            #[serde(rename = "index")]
            pub index: Vec<JsUInt>,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: Vec<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DocumentSnapshot {
            #[serde(rename = "documentURL")]
            pub document_url: StringIndex,
            #[serde(rename = "title")]
            pub title: StringIndex,
            #[serde(rename = "baseURL")]
            pub base_url: StringIndex,
            #[serde(rename = "contentLanguage")]
            pub content_language: StringIndex,
            #[serde(rename = "encodingName")]
            pub encoding_name: StringIndex,
            #[serde(rename = "publicId")]
            pub public_id: StringIndex,
            #[serde(rename = "systemId")]
            pub system_id: StringIndex,
            #[serde(rename = "frameId")]
            pub frame_id: StringIndex,
            #[serde(rename = "nodes")]
            pub nodes: NodeTreeSnapshot,
            #[serde(rename = "layout")]
            pub layout: LayoutTreeSnapshot,
            #[serde(rename = "textBoxes")]
            pub text_boxes: TextBoxSnapshot,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "scrollOffsetX")]
            pub scroll_offset_x: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "scrollOffsetY")]
            pub scroll_offset_y: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "contentWidth")]
            pub content_width: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "contentHeight")]
            pub content_height: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct NodeTreeSnapshot {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "parentIndex")]
            pub parent_index: Option<Vec<JsUInt>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "nodeType")]
            pub node_type: Option<Vec<JsUInt>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "shadowRootType")]
            pub shadow_root_Type: Option<RareStringData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeName")]
            pub node_name: Option<Vec<StringIndex>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeValue")]
            pub node_value: Option<Vec<StringIndex>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: Option<Vec<DOM::BackendNodeId>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "attributes")]
            pub attributes: Option<Vec<ArrayOfStrings>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "textValue")]
            pub text_value: Option<RareStringData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "inputValue")]
            pub input_value: Option<RareStringData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "inputChecked")]
            pub input_checked: Option<RareBooleanData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "optionSelected")]
            pub option_selected: Option<RareBooleanData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "contentDocumentIndex")]
            pub content_document_index: Option<RareIntegerData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "pseudoType")]
            pub pseudo_Type: Option<RareStringData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "isClickable")]
            pub is_clickable: Option<RareBooleanData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "currentSourceURL")]
            pub current_source_url: Option<RareStringData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "originURL")]
            pub origin_url: Option<RareStringData>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct LayoutTreeSnapshot {
            #[serde(default)]
            #[serde(rename = "nodeIndex")]
            pub node_index: Vec<JsUInt>,
            #[serde(rename = "styles")]
            pub styles: Vec<ArrayOfStrings>,
            #[serde(rename = "bounds")]
            pub bounds: Vec<Rectangle>,
            #[serde(rename = "text")]
            pub text: Vec<StringIndex>,
            #[serde(rename = "stackingContexts")]
            pub stacking_contexts: RareBooleanData,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "paintOrders")]
            pub paint_orders: Option<Vec<JsUInt>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "offsetRects")]
            pub offset_rects: Option<Vec<Rectangle>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "scrollRects")]
            pub scroll_rects: Option<Vec<Rectangle>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "clientRects")]
            pub client_rects: Option<Vec<Rectangle>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "blendedBackgroundColors")]
            pub blended_background_colors: Option<Vec<StringIndex>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "textColorOpacities")]
            pub text_color_opacities: Option<Vec<JsFloat>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TextBoxSnapshot {
            #[serde(default)]
            #[serde(rename = "layoutIndex")]
            pub layout_index: Vec<JsUInt>,
            #[serde(rename = "bounds")]
            pub bounds: Vec<Rectangle>,
            #[serde(default)]
            #[serde(rename = "start")]
            pub start: Vec<JsUInt>,
            #[serde(default)]
            #[serde(rename = "length")]
            pub length: Vec<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetSnapshot {
            #[serde(default)]
            #[serde(rename = "computedStyleWhitelist")]
            pub computed_style_whitelist: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "includeEventListeners")]
            pub include_event_listeners: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "includePaintOrder")]
            pub include_paint_order: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "includeUserAgentShadowTree")]
            pub include_user_agent_shadow_tree: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CaptureSnapshot {
            #[serde(default)]
            #[serde(rename = "computedStyles")]
            pub computed_styles: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "includePaintOrder")]
            pub include_paint_order: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "includeDOMRects")]
            pub include_dom_rects: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "includeBlendedBackgroundColors")]
            pub include_blended_background_colors: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "includeTextColorOpacities")]
            pub include_text_color_opacities: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetSnapshotReturnObject {
            #[serde(rename = "domNodes")]
            pub dom_nodes: Vec<DOMNode>,
            #[serde(rename = "layoutTreeNodes")]
            pub layout_tree_nodes: Vec<LayoutTreeNode>,
            #[serde(rename = "computedStyles")]
            pub computed_styles: Vec<ComputedStyle>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CaptureSnapshotReturnObject {
            #[serde(rename = "documents")]
            pub documents: Vec<DocumentSnapshot>,
            #[serde(rename = "strings")]
            pub strings: Vec<String>,
        }
        impl Method for Disable {
            const NAME: &'static str = "DOMSnapshot.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "DOMSnapshot.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetSnapshot {
            const NAME: &'static str = "DOMSnapshot.getSnapshot";
            type ReturnObject = GetSnapshotReturnObject;
        }
        impl Method for CaptureSnapshot {
            const NAME: &'static str = "DOMSnapshot.captureSnapshot";
            type ReturnObject = CaptureSnapshotReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod DOMStorage {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type Item = Vec<String>;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StorageId {
            #[serde(default)]
            #[serde(rename = "securityOrigin")]
            pub security_origin: String,
            #[serde(default)]
            #[serde(rename = "isLocalStorage")]
            pub is_local_storage: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Clear {
            #[serde(rename = "storageId")]
            pub storage_id: StorageId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetDOMStorageItems {
            #[serde(rename = "storageId")]
            pub storage_id: StorageId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RemoveDOMStorageItem {
            #[serde(rename = "storageId")]
            pub storage_id: StorageId,
            #[serde(default)]
            #[serde(rename = "key")]
            pub key: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetDOMStorageItem {
            #[serde(rename = "storageId")]
            pub storage_id: StorageId,
            #[serde(default)]
            #[serde(rename = "key")]
            pub key: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetDOMStorageItemsReturnObject {
            #[serde(rename = "entries")]
            pub entries: Vec<Item>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveDOMStorageItemReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDOMStorageItemReturnObject {}
        impl Method for Clear {
            const NAME: &'static str = "DOMStorage.clear";
            type ReturnObject = ClearReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "DOMStorage.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "DOMStorage.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetDOMStorageItems {
            const NAME: &'static str = "DOMStorage.getDOMStorageItems";
            type ReturnObject = GetDOMStorageItemsReturnObject;
        }
        impl Method for RemoveDOMStorageItem {
            const NAME: &'static str = "DOMStorage.removeDOMStorageItem";
            type ReturnObject = RemoveDOMStorageItemReturnObject;
        }
        impl Method for SetDOMStorageItem {
            const NAME: &'static str = "DOMStorage.setDOMStorageItem";
            type ReturnObject = SetDOMStorageItemReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DomStorageItemAddedEvent {
                pub params: DomStorageItemAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DomStorageItemAddedEventParams {
                #[serde(rename = "storageId")]
                pub storage_id: super::StorageId,
                #[serde(default)]
                #[serde(rename = "key")]
                pub key: String,
                #[serde(default)]
                #[serde(rename = "newValue")]
                pub new_value: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DomStorageItemRemovedEvent {
                pub params: DomStorageItemRemovedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DomStorageItemRemovedEventParams {
                #[serde(rename = "storageId")]
                pub storage_id: super::StorageId,
                #[serde(default)]
                #[serde(rename = "key")]
                pub key: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DomStorageItemUpdatedEvent {
                pub params: DomStorageItemUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DomStorageItemUpdatedEventParams {
                #[serde(rename = "storageId")]
                pub storage_id: super::StorageId,
                #[serde(default)]
                #[serde(rename = "key")]
                pub key: String,
                #[serde(default)]
                #[serde(rename = "oldValue")]
                pub old_value: String,
                #[serde(default)]
                #[serde(rename = "newValue")]
                pub new_value: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DomStorageItemsClearedEvent {
                pub params: DomStorageItemsClearedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DomStorageItemsClearedEventParams {
                #[serde(rename = "storageId")]
                pub storage_id: super::StorageId,
            }
        }
    }
    pub mod Database {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type DatabaseId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Database {
            #[serde(rename = "id")]
            pub id: DatabaseId,
            #[serde(default)]
            #[serde(rename = "domain")]
            pub domain: String,
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "version")]
            pub version: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Error {
            #[serde(default)]
            #[serde(rename = "message")]
            pub message: String,
            #[serde(default)]
            #[serde(rename = "code")]
            pub code: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ExecuteSQL {
            #[serde(rename = "databaseId")]
            pub database_id: DatabaseId,
            #[serde(default)]
            #[serde(rename = "query")]
            pub query: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetDatabaseTableNames {
            #[serde(rename = "databaseId")]
            pub database_id: DatabaseId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ExecuteSQLReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "columnNames")]
            pub column_names: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "values")]
            pub values: Option<Vec<Json>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "sqlError")]
            pub sql_error: Option<Error>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetDatabaseTableNamesReturnObject {
            #[serde(rename = "tableNames")]
            pub table_names: Vec<String>,
        }
        impl Method for Disable {
            const NAME: &'static str = "Database.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Database.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for ExecuteSQL {
            const NAME: &'static str = "Database.executeSQL";
            type ReturnObject = ExecuteSQLReturnObject;
        }
        impl Method for GetDatabaseTableNames {
            const NAME: &'static str = "Database.getDatabaseTableNames";
            type ReturnObject = GetDatabaseTableNamesReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AddDatabaseEvent {
                pub params: AddDatabaseEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AddDatabaseEventParams {
                #[serde(rename = "database")]
                pub database: super::Database,
            }
        }
    }
    pub mod DeviceOrientation {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDeviceOrientationOverride(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetDeviceOrientationOverride {
            #[serde(default)]
            #[serde(rename = "alpha")]
            pub alpha: JsFloat,
            #[serde(default)]
            #[serde(rename = "beta")]
            pub beta: JsFloat,
            #[serde(default)]
            #[serde(rename = "gamma")]
            pub gamma: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDeviceOrientationOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDeviceOrientationOverrideReturnObject {}
        impl Method for ClearDeviceOrientationOverride {
            const NAME: &'static str = "DeviceOrientation.clearDeviceOrientationOverride";
            type ReturnObject = ClearDeviceOrientationOverrideReturnObject;
        }
        impl Method for SetDeviceOrientationOverride {
            const NAME: &'static str = "DeviceOrientation.setDeviceOrientationOverride";
            type ReturnObject = SetDeviceOrientationOverrideReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod Emulation {
        use super::types::*;
        use super::Network;
        use super::Page;
        use super::Runtime;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ScreenOrientationType {
            #[serde(rename = "portraitPrimary")]
            PortraitPrimary,
            #[serde(rename = "portraitSecondary")]
            PortraitSecondary,
            #[serde(rename = "landscapePrimary")]
            LandscapePrimary,
            #[serde(rename = "landscapeSecondary")]
            LandscapeSecondary,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum DisplayFeatureOrientation {
            #[serde(rename = "vertical")]
            Vertical,
            #[serde(rename = "horizontal")]
            Horizontal,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum VirtualTimePolicy {
            #[serde(rename = "advance")]
            Advance,
            #[serde(rename = "pause")]
            Pause,
            #[serde(rename = "pauseIfNetworkFetchesPending")]
            PauseIfNetworkFetchesPending,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum DisabledImageType {
            #[serde(rename = "avif")]
            Avif,
            #[serde(rename = "jxl")]
            Jxl,
            #[serde(rename = "webp")]
            Webp,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SetEmitTouchEventsForMouseConfigurationOption {
            #[serde(rename = "mobile")]
            Mobile,
            #[serde(rename = "desktop")]
            Desktop,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SetEmulatedVisionDeficiencyTypeOption {
            #[serde(rename = "none")]
            None,
            #[serde(rename = "achromatopsia")]
            Achromatopsia,
            #[serde(rename = "blurredVision")]
            BlurredVision,
            #[serde(rename = "deuteranopia")]
            Deuteranopia,
            #[serde(rename = "protanopia")]
            Protanopia,
            #[serde(rename = "tritanopia")]
            Tritanopia,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ScreenOrientation {
            #[serde(rename = "type")]
            pub Type: ScreenOrientationType,
            #[serde(default)]
            #[serde(rename = "angle")]
            pub angle: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DisplayFeature {
            #[serde(rename = "orientation")]
            pub orientation: DisplayFeatureOrientation,
            #[serde(default)]
            #[serde(rename = "offset")]
            pub offset: JsUInt,
            #[serde(default)]
            #[serde(rename = "maskLength")]
            pub mask_length: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct MediaFeature {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct UserAgentBrandVersion {
            #[serde(default)]
            #[serde(rename = "brand")]
            pub brand: String,
            #[serde(default)]
            #[serde(rename = "version")]
            pub version: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct UserAgentMetadata {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "brands")]
            pub brands: Option<Vec<UserAgentBrandVersion>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "fullVersionList")]
            pub full_version_list: Option<Vec<UserAgentBrandVersion>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "fullVersion")]
            pub full_version: Option<String>,
            #[serde(default)]
            #[serde(rename = "platform")]
            pub platform: String,
            #[serde(default)]
            #[serde(rename = "platformVersion")]
            pub platform_version: String,
            #[serde(default)]
            #[serde(rename = "architecture")]
            pub architecture: String,
            #[serde(default)]
            #[serde(rename = "model")]
            pub model: String,
            #[serde(default)]
            #[serde(rename = "mobile")]
            pub mobile: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CanEmulate(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDeviceMetricsOverride(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearGeolocationOverride(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResetPageScaleFactor(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetFocusEmulationEnabled {
            #[serde(default)]
            #[serde(rename = "enabled")]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetAutoDarkModeOverride {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "enabled")]
            pub enabled: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetCPUThrottlingRate {
            #[serde(default)]
            #[serde(rename = "rate")]
            pub rate: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetDefaultBackgroundColorOverride {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "color")]
            pub color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetDeviceMetricsOverride {
            #[serde(default)]
            #[serde(rename = "width")]
            pub width: JsUInt,
            #[serde(default)]
            #[serde(rename = "height")]
            pub height: JsUInt,
            #[serde(default)]
            #[serde(rename = "deviceScaleFactor")]
            pub device_scale_factor: JsFloat,
            #[serde(default)]
            #[serde(rename = "mobile")]
            pub mobile: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "scale")]
            pub scale: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "screenWidth")]
            pub screen_width: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "screenHeight")]
            pub screen_height: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "positionX")]
            pub position_x: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "positionY")]
            pub position_y: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "dontSetVisibleSize")]
            pub dont_set_visible_size: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "screenOrientation")]
            pub screen_orientation: Option<ScreenOrientation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "viewport")]
            pub viewport: Option<Page::Viewport>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "displayFeature")]
            pub display_feature: Option<DisplayFeature>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetScrollbarsHidden {
            #[serde(default)]
            #[serde(rename = "hidden")]
            pub hidden: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetDocumentCookieDisabled {
            #[serde(default)]
            #[serde(rename = "disabled")]
            pub disabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetEmitTouchEventsForMouse {
            #[serde(default)]
            #[serde(rename = "enabled")]
            pub enabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "configuration")]
            pub configuration: Option<SetEmitTouchEventsForMouseConfigurationOption>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetEmulatedMedia {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "media")]
            pub media: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "features")]
            pub features: Option<Vec<MediaFeature>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetEmulatedVisionDeficiency {
            #[serde(rename = "type")]
            pub Type: SetEmulatedVisionDeficiencyTypeOption,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetGeolocationOverride {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "latitude")]
            pub latitude: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "longitude")]
            pub longitude: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "accuracy")]
            pub accuracy: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetIdleOverride {
            #[serde(default)]
            #[serde(rename = "isUserActive")]
            pub is_user_active: bool,
            #[serde(default)]
            #[serde(rename = "isScreenUnlocked")]
            pub is_screen_unlocked: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearIdleOverride(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetNavigatorOverrides {
            #[serde(default)]
            #[serde(rename = "platform")]
            pub platform: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetPageScaleFactor {
            #[serde(default)]
            #[serde(rename = "pageScaleFactor")]
            pub page_scale_factor: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetScriptExecutionDisabled {
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetTouchEmulationEnabled {
            #[serde(default)]
            #[serde(rename = "enabled")]
            pub enabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "maxTouchPoints")]
            pub max_touch_points: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetVirtualTimePolicy {
            #[serde(rename = "policy")]
            pub policy: VirtualTimePolicy,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "budget")]
            pub budget: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "maxVirtualTimeTaskStarvationCount")]
            pub max_virtual_time_task_starvation_count: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "waitForNavigation")]
            pub wait_for_navigation: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "initialVirtualTime")]
            pub initial_virtual_time: Option<Network::TimeSinceEpoch>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetLocaleOverride {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "locale")]
            pub locale: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetTimezoneOverride {
            #[serde(default)]
            #[serde(rename = "timezoneId")]
            pub timezone_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetVisibleSize {
            #[serde(default)]
            #[serde(rename = "width")]
            pub width: JsUInt,
            #[serde(default)]
            #[serde(rename = "height")]
            pub height: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetDisabledImageTypes {
            #[serde(rename = "imageTypes")]
            pub image_Types: Vec<DisabledImageType>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetUserAgentOverride {
            #[serde(default)]
            #[serde(rename = "userAgent")]
            pub user_agent: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "acceptLanguage")]
            pub accept_language: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "platform")]
            pub platform: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "userAgentMetadata")]
            pub user_agent_metadata: Option<UserAgentMetadata>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CanEmulateReturnObject {
            #[serde(default)]
            #[serde(rename = "result")]
            pub result: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDeviceMetricsOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearGeolocationOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResetPageScaleFactorReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetFocusEmulationEnabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAutoDarkModeOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetCPUThrottlingRateReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDefaultBackgroundColorOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDeviceMetricsOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetScrollbarsHiddenReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDocumentCookieDisabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetEmitTouchEventsForMouseReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetEmulatedMediaReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetEmulatedVisionDeficiencyReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetGeolocationOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetIdleOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearIdleOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetNavigatorOverridesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPageScaleFactorReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetScriptExecutionDisabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetTouchEmulationEnabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetVirtualTimePolicyReturnObject {
            #[serde(default)]
            #[serde(rename = "virtualTimeTicksBase")]
            pub virtual_time_ticks_base: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetLocaleOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetTimezoneOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetVisibleSizeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDisabledImageTypesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetUserAgentOverrideReturnObject {}
        impl Method for CanEmulate {
            const NAME: &'static str = "Emulation.canEmulate";
            type ReturnObject = CanEmulateReturnObject;
        }
        impl Method for ClearDeviceMetricsOverride {
            const NAME: &'static str = "Emulation.clearDeviceMetricsOverride";
            type ReturnObject = ClearDeviceMetricsOverrideReturnObject;
        }
        impl Method for ClearGeolocationOverride {
            const NAME: &'static str = "Emulation.clearGeolocationOverride";
            type ReturnObject = ClearGeolocationOverrideReturnObject;
        }
        impl Method for ResetPageScaleFactor {
            const NAME: &'static str = "Emulation.resetPageScaleFactor";
            type ReturnObject = ResetPageScaleFactorReturnObject;
        }
        impl Method for SetFocusEmulationEnabled {
            const NAME: &'static str = "Emulation.setFocusEmulationEnabled";
            type ReturnObject = SetFocusEmulationEnabledReturnObject;
        }
        impl Method for SetAutoDarkModeOverride {
            const NAME: &'static str = "Emulation.setAutoDarkModeOverride";
            type ReturnObject = SetAutoDarkModeOverrideReturnObject;
        }
        impl Method for SetCPUThrottlingRate {
            const NAME: &'static str = "Emulation.setCPUThrottlingRate";
            type ReturnObject = SetCPUThrottlingRateReturnObject;
        }
        impl Method for SetDefaultBackgroundColorOverride {
            const NAME: &'static str = "Emulation.setDefaultBackgroundColorOverride";
            type ReturnObject = SetDefaultBackgroundColorOverrideReturnObject;
        }
        impl Method for SetDeviceMetricsOverride {
            const NAME: &'static str = "Emulation.setDeviceMetricsOverride";
            type ReturnObject = SetDeviceMetricsOverrideReturnObject;
        }
        impl Method for SetScrollbarsHidden {
            const NAME: &'static str = "Emulation.setScrollbarsHidden";
            type ReturnObject = SetScrollbarsHiddenReturnObject;
        }
        impl Method for SetDocumentCookieDisabled {
            const NAME: &'static str = "Emulation.setDocumentCookieDisabled";
            type ReturnObject = SetDocumentCookieDisabledReturnObject;
        }
        impl Method for SetEmitTouchEventsForMouse {
            const NAME: &'static str = "Emulation.setEmitTouchEventsForMouse";
            type ReturnObject = SetEmitTouchEventsForMouseReturnObject;
        }
        impl Method for SetEmulatedMedia {
            const NAME: &'static str = "Emulation.setEmulatedMedia";
            type ReturnObject = SetEmulatedMediaReturnObject;
        }
        impl Method for SetEmulatedVisionDeficiency {
            const NAME: &'static str = "Emulation.setEmulatedVisionDeficiency";
            type ReturnObject = SetEmulatedVisionDeficiencyReturnObject;
        }
        impl Method for SetGeolocationOverride {
            const NAME: &'static str = "Emulation.setGeolocationOverride";
            type ReturnObject = SetGeolocationOverrideReturnObject;
        }
        impl Method for SetIdleOverride {
            const NAME: &'static str = "Emulation.setIdleOverride";
            type ReturnObject = SetIdleOverrideReturnObject;
        }
        impl Method for ClearIdleOverride {
            const NAME: &'static str = "Emulation.clearIdleOverride";
            type ReturnObject = ClearIdleOverrideReturnObject;
        }
        impl Method for SetNavigatorOverrides {
            const NAME: &'static str = "Emulation.setNavigatorOverrides";
            type ReturnObject = SetNavigatorOverridesReturnObject;
        }
        impl Method for SetPageScaleFactor {
            const NAME: &'static str = "Emulation.setPageScaleFactor";
            type ReturnObject = SetPageScaleFactorReturnObject;
        }
        impl Method for SetScriptExecutionDisabled {
            const NAME: &'static str = "Emulation.setScriptExecutionDisabled";
            type ReturnObject = SetScriptExecutionDisabledReturnObject;
        }
        impl Method for SetTouchEmulationEnabled {
            const NAME: &'static str = "Emulation.setTouchEmulationEnabled";
            type ReturnObject = SetTouchEmulationEnabledReturnObject;
        }
        impl Method for SetVirtualTimePolicy {
            const NAME: &'static str = "Emulation.setVirtualTimePolicy";
            type ReturnObject = SetVirtualTimePolicyReturnObject;
        }
        impl Method for SetLocaleOverride {
            const NAME: &'static str = "Emulation.setLocaleOverride";
            type ReturnObject = SetLocaleOverrideReturnObject;
        }
        impl Method for SetTimezoneOverride {
            const NAME: &'static str = "Emulation.setTimezoneOverride";
            type ReturnObject = SetTimezoneOverrideReturnObject;
        }
        impl Method for SetVisibleSize {
            const NAME: &'static str = "Emulation.setVisibleSize";
            type ReturnObject = SetVisibleSizeReturnObject;
        }
        impl Method for SetDisabledImageTypes {
            const NAME: &'static str = "Emulation.setDisabledImageTypes";
            type ReturnObject = SetDisabledImageTypesReturnObject;
        }
        impl Method for SetUserAgentOverride {
            const NAME: &'static str = "Emulation.setUserAgentOverride";
            type ReturnObject = SetUserAgentOverrideReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct VirtualTimeBudgetExpiredEvent(pub Option<serde_json::Value>);
        }
    }
    pub mod HeadlessExperimental {
        use super::types::*;
        use super::Page;
        use super::Runtime;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ScreenshotParamsFormat {
            #[serde(rename = "jpeg")]
            Jpeg,
            #[serde(rename = "png")]
            Png,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ScreenshotParams {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "format")]
            pub format: Option<ScreenshotParamsFormat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "quality")]
            pub quality: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct BeginFrame {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "frameTimeTicks")]
            pub frame_time_ticks: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "interval")]
            pub interval: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "noDisplayUpdates")]
            pub no_display_updates: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "screenshot")]
            pub screenshot: Option<ScreenshotParams>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct BeginFrameReturnObject {
            #[serde(default)]
            #[serde(rename = "hasDamage")]
            pub has_damage: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "screenshotData")]
            pub screenshot_data: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        impl Method for BeginFrame {
            const NAME: &'static str = "HeadlessExperimental.beginFrame";
            type ReturnObject = BeginFrameReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "HeadlessExperimental.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "HeadlessExperimental.enable";
            type ReturnObject = EnableReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NeedsBeginFramesChangedEvent {
                pub params: NeedsBeginFramesChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NeedsBeginFramesChangedEventParams {
                #[serde(default)]
                #[serde(rename = "needsBeginFrames")]
                pub needs_begin_frames: bool,
            }
        }
    }
    pub mod IO {
        use super::types::*;
        use super::Runtime;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type StreamHandle = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Close {
            #[serde(rename = "handle")]
            pub handle: StreamHandle,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Read {
            #[serde(rename = "handle")]
            pub handle: StreamHandle,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "offset")]
            pub offset: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "size")]
            pub size: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ResolveBlob {
            #[serde(rename = "objectId")]
            pub object_id: Runtime::RemoteObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CloseReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ReadReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "base64Encoded")]
            pub base_64_encoded: Option<bool>,
            #[serde(default)]
            #[serde(rename = "data")]
            pub data: String,
            #[serde(default)]
            #[serde(rename = "eof")]
            pub eof: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ResolveBlobReturnObject {
            #[serde(default)]
            #[serde(rename = "uuid")]
            pub uuid: String,
        }
        impl Method for Close {
            const NAME: &'static str = "IO.close";
            type ReturnObject = CloseReturnObject;
        }
        impl Method for Read {
            const NAME: &'static str = "IO.read";
            type ReturnObject = ReadReturnObject;
        }
        impl Method for ResolveBlob {
            const NAME: &'static str = "IO.resolveBlob";
            type ReturnObject = ResolveBlobReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod IndexedDB {
        use super::types::*;
        use super::Runtime;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum KeyType {
            #[serde(rename = "number")]
            Number,
            #[serde(rename = "string")]
            String,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "array")]
            Array,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum KeyPathType {
            #[serde(rename = "null")]
            Null,
            #[serde(rename = "string")]
            String,
            #[serde(rename = "array")]
            Array,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DatabaseWithObjectStores {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "version")]
            pub version: JsFloat,
            #[serde(rename = "objectStores")]
            pub object_stores: Vec<ObjectStore>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ObjectStore {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(rename = "keyPath")]
            pub key_path: KeyPath,
            #[serde(default)]
            #[serde(rename = "autoIncrement")]
            pub auto_increment: bool,
            #[serde(rename = "indexes")]
            pub indexes: Vec<ObjectStoreIndex>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ObjectStoreIndex {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(rename = "keyPath")]
            pub key_path: KeyPath,
            #[serde(default)]
            #[serde(rename = "unique")]
            pub unique: bool,
            #[serde(default)]
            #[serde(rename = "multiEntry")]
            pub multi_entry: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Key {
            #[serde(rename = "type")]
            pub Type: KeyType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "number")]
            pub number: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "string")]
            pub string: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "date")]
            pub date: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "array")]
            pub array: Option<Vec<Key>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct KeyRange {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "lower")]
            pub lower: Option<Key>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "upper")]
            pub upper: Option<Key>,
            #[serde(default)]
            #[serde(rename = "lowerOpen")]
            pub lower_open: bool,
            #[serde(default)]
            #[serde(rename = "upperOpen")]
            pub upper_open: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DataEntry {
            #[serde(rename = "key")]
            pub key: Runtime::RemoteObject,
            #[serde(rename = "primaryKey")]
            pub primary_key: Runtime::RemoteObject,
            #[serde(rename = "value")]
            pub value: Runtime::RemoteObject,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct KeyPath {
            #[serde(rename = "type")]
            pub Type: KeyPathType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "string")]
            pub string: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "array")]
            pub array: Option<Vec<String>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ClearObjectStore {
            #[serde(default)]
            #[serde(rename = "securityOrigin")]
            pub security_origin: String,
            #[serde(default)]
            #[serde(rename = "databaseName")]
            pub database_name: String,
            #[serde(default)]
            #[serde(rename = "objectStoreName")]
            pub object_store_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DeleteDatabase {
            #[serde(default)]
            #[serde(rename = "securityOrigin")]
            pub security_origin: String,
            #[serde(default)]
            #[serde(rename = "databaseName")]
            pub database_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DeleteObjectStoreEntries {
            #[serde(default)]
            #[serde(rename = "securityOrigin")]
            pub security_origin: String,
            #[serde(default)]
            #[serde(rename = "databaseName")]
            pub database_name: String,
            #[serde(default)]
            #[serde(rename = "objectStoreName")]
            pub object_store_name: String,
            #[serde(rename = "keyRange")]
            pub key_range: KeyRange,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestData {
            #[serde(default)]
            #[serde(rename = "securityOrigin")]
            pub security_origin: String,
            #[serde(default)]
            #[serde(rename = "databaseName")]
            pub database_name: String,
            #[serde(default)]
            #[serde(rename = "objectStoreName")]
            pub object_store_name: String,
            #[serde(default)]
            #[serde(rename = "indexName")]
            pub index_name: String,
            #[serde(default)]
            #[serde(rename = "skipCount")]
            pub skip_count: JsUInt,
            #[serde(default)]
            #[serde(rename = "pageSize")]
            pub page_size: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "keyRange")]
            pub key_range: Option<KeyRange>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetMetadata {
            #[serde(default)]
            #[serde(rename = "securityOrigin")]
            pub security_origin: String,
            #[serde(default)]
            #[serde(rename = "databaseName")]
            pub database_name: String,
            #[serde(default)]
            #[serde(rename = "objectStoreName")]
            pub object_store_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestDatabase {
            #[serde(default)]
            #[serde(rename = "securityOrigin")]
            pub security_origin: String,
            #[serde(default)]
            #[serde(rename = "databaseName")]
            pub database_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestDatabaseNames {
            #[serde(default)]
            #[serde(rename = "securityOrigin")]
            pub security_origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearObjectStoreReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteDatabaseReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteObjectStoreEntriesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestDataReturnObject {
            #[serde(rename = "objectStoreDataEntries")]
            pub object_store_data_entries: Vec<DataEntry>,
            #[serde(default)]
            #[serde(rename = "hasMore")]
            pub has_more: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetMetadataReturnObject {
            #[serde(default)]
            #[serde(rename = "entriesCount")]
            pub entries_count: JsFloat,
            #[serde(default)]
            #[serde(rename = "keyGeneratorValue")]
            pub key_generator_value: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestDatabaseReturnObject {
            #[serde(rename = "databaseWithObjectStores")]
            pub database_with_object_stores: DatabaseWithObjectStores,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestDatabaseNamesReturnObject {
            #[serde(rename = "databaseNames")]
            pub database_names: Vec<String>,
        }
        impl Method for ClearObjectStore {
            const NAME: &'static str = "IndexedDB.clearObjectStore";
            type ReturnObject = ClearObjectStoreReturnObject;
        }
        impl Method for DeleteDatabase {
            const NAME: &'static str = "IndexedDB.deleteDatabase";
            type ReturnObject = DeleteDatabaseReturnObject;
        }
        impl Method for DeleteObjectStoreEntries {
            const NAME: &'static str = "IndexedDB.deleteObjectStoreEntries";
            type ReturnObject = DeleteObjectStoreEntriesReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "IndexedDB.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "IndexedDB.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for RequestData {
            const NAME: &'static str = "IndexedDB.requestData";
            type ReturnObject = RequestDataReturnObject;
        }
        impl Method for GetMetadata {
            const NAME: &'static str = "IndexedDB.getMetadata";
            type ReturnObject = GetMetadataReturnObject;
        }
        impl Method for RequestDatabase {
            const NAME: &'static str = "IndexedDB.requestDatabase";
            type ReturnObject = RequestDatabaseReturnObject;
        }
        impl Method for RequestDatabaseNames {
            const NAME: &'static str = "IndexedDB.requestDatabaseNames";
            type ReturnObject = RequestDatabaseNamesReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod Input {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type TimeSinceEpoch = JsFloat;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum GestureSourceType {
            #[serde(rename = "default")]
            Default,
            #[serde(rename = "touch")]
            Touch,
            #[serde(rename = "mouse")]
            Mouse,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum MouseButton {
            #[serde(rename = "none")]
            None,
            #[serde(rename = "left")]
            Left,
            #[serde(rename = "middle")]
            Middle,
            #[serde(rename = "right")]
            Right,
            #[serde(rename = "back")]
            Back,
            #[serde(rename = "forward")]
            Forward,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum DispatchDragEventTypeOption {
            #[serde(rename = "dragEnter")]
            DragEnter,
            #[serde(rename = "dragOver")]
            DragOver,
            #[serde(rename = "drop")]
            Drop,
            #[serde(rename = "dragCancel")]
            DragCancel,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum DispatchKeyEventTypeOption {
            #[serde(rename = "keyDown")]
            KeyDown,
            #[serde(rename = "keyUp")]
            KeyUp,
            #[serde(rename = "rawKeyDown")]
            RawKeyDown,
            #[serde(rename = "char")]
            Char,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum DispatchMouseEventTypeOption {
            #[serde(rename = "mousePressed")]
            MousePressed,
            #[serde(rename = "mouseReleased")]
            MouseReleased,
            #[serde(rename = "mouseMoved")]
            MouseMoved,
            #[serde(rename = "mouseWheel")]
            MouseWheel,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum DispatchMouseEventPointer_TypeOption {
            #[serde(rename = "mouse")]
            Mouse,
            #[serde(rename = "pen")]
            Pen,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum DispatchTouchEventTypeOption {
            #[serde(rename = "touchStart")]
            TouchStart,
            #[serde(rename = "touchEnd")]
            TouchEnd,
            #[serde(rename = "touchMove")]
            TouchMove,
            #[serde(rename = "touchCancel")]
            TouchCancel,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum EmulateTouchFromMouseEventTypeOption {
            #[serde(rename = "mousePressed")]
            MousePressed,
            #[serde(rename = "mouseReleased")]
            MouseReleased,
            #[serde(rename = "mouseMoved")]
            MouseMoved,
            #[serde(rename = "mouseWheel")]
            MouseWheel,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TouchPoint {
            #[serde(default)]
            #[serde(rename = "x")]
            pub x: JsFloat,
            #[serde(default)]
            #[serde(rename = "y")]
            pub y: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "radiusX")]
            pub radius_x: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "radiusY")]
            pub radius_y: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "rotationAngle")]
            pub rotation_angle: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "force")]
            pub force: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "tangentialPressure")]
            pub tangential_pressure: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "tiltX")]
            pub tilt_x: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "tiltY")]
            pub tilt_y: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "twist")]
            pub twist: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "id")]
            pub id: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DragDataItem {
            #[serde(default)]
            #[serde(rename = "mimeType")]
            pub mime_type: String,
            #[serde(default)]
            #[serde(rename = "data")]
            pub data: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "title")]
            pub title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "baseURL")]
            pub base_url: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DragData {
            #[serde(rename = "items")]
            pub items: Vec<DragDataItem>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "files")]
            pub files: Option<Vec<String>>,
            #[serde(default)]
            #[serde(rename = "dragOperationsMask")]
            pub drag_operations_mask: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DispatchDragEvent {
            #[serde(rename = "type")]
            pub Type: DispatchDragEventTypeOption,
            #[serde(default)]
            #[serde(rename = "x")]
            pub x: JsFloat,
            #[serde(default)]
            #[serde(rename = "y")]
            pub y: JsFloat,
            #[serde(rename = "data")]
            pub data: DragData,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "modifiers")]
            pub modifiers: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DispatchKeyEvent {
            #[serde(rename = "type")]
            pub Type: DispatchKeyEventTypeOption,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "modifiers")]
            pub modifiers: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "timestamp")]
            pub timestamp: Option<TimeSinceEpoch>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "unmodifiedText")]
            pub unmodified_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "keyIdentifier")]
            pub key_identifier: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "code")]
            pub code: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "key")]
            pub key: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "windowsVirtualKeyCode")]
            pub windows_virtual_key_code: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "nativeVirtualKeyCode")]
            pub native_virtual_key_code: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "autoRepeat")]
            pub auto_repeat: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "isKeypad")]
            pub is_keypad: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "isSystemKey")]
            pub is_system_key: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "location")]
            pub location: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "commands")]
            pub commands: Option<Vec<String>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct InsertText {
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ImeSetComposition {
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: String,
            #[serde(default)]
            #[serde(rename = "selectionStart")]
            pub selection_start: JsUInt,
            #[serde(default)]
            #[serde(rename = "selectionEnd")]
            pub selection_end: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "replacementStart")]
            pub replacement_start: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "replacementEnd")]
            pub replacement_end: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DispatchMouseEvent {
            #[serde(rename = "type")]
            pub Type: DispatchMouseEventTypeOption,
            #[serde(default)]
            #[serde(rename = "x")]
            pub x: JsFloat,
            #[serde(default)]
            #[serde(rename = "y")]
            pub y: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "modifiers")]
            pub modifiers: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "timestamp")]
            pub timestamp: Option<TimeSinceEpoch>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "button")]
            pub button: Option<MouseButton>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "buttons")]
            pub buttons: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "clickCount")]
            pub click_count: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "force")]
            pub force: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "tangentialPressure")]
            pub tangential_pressure: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "tiltX")]
            pub tilt_x: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "tiltY")]
            pub tilt_y: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "twist")]
            pub twist: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "deltaX")]
            pub delta_x: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "deltaY")]
            pub delta_y: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "pointerType")]
            pub pointer_Type: Option<DispatchMouseEventPointer_TypeOption>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DispatchTouchEvent {
            #[serde(rename = "type")]
            pub Type: DispatchTouchEventTypeOption,
            #[serde(rename = "touchPoints")]
            pub touch_points: Vec<TouchPoint>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "modifiers")]
            pub modifiers: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "timestamp")]
            pub timestamp: Option<TimeSinceEpoch>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct EmulateTouchFromMouseEvent {
            #[serde(rename = "type")]
            pub Type: EmulateTouchFromMouseEventTypeOption,
            #[serde(default)]
            #[serde(rename = "x")]
            pub x: JsUInt,
            #[serde(default)]
            #[serde(rename = "y")]
            pub y: JsUInt,
            #[serde(rename = "button")]
            pub button: MouseButton,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "timestamp")]
            pub timestamp: Option<TimeSinceEpoch>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "deltaX")]
            pub delta_x: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "deltaY")]
            pub delta_y: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "modifiers")]
            pub modifiers: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "clickCount")]
            pub click_count: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetIgnoreInputEvents {
            #[serde(default)]
            #[serde(rename = "ignore")]
            pub ignore: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetInterceptDrags {
            #[serde(default)]
            #[serde(rename = "enabled")]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SynthesizePinchGesture {
            #[serde(default)]
            #[serde(rename = "x")]
            pub x: JsFloat,
            #[serde(default)]
            #[serde(rename = "y")]
            pub y: JsFloat,
            #[serde(default)]
            #[serde(rename = "scaleFactor")]
            pub scale_factor: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "relativeSpeed")]
            pub relative_speed: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "gestureSourceType")]
            pub gesture_source_Type: Option<GestureSourceType>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SynthesizeScrollGesture {
            #[serde(default)]
            #[serde(rename = "x")]
            pub x: JsFloat,
            #[serde(default)]
            #[serde(rename = "y")]
            pub y: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "xDistance")]
            pub x_distance: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "yDistance")]
            pub y_distance: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "xOverscroll")]
            pub x_overscroll: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "yOverscroll")]
            pub y_overscroll: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "preventFling")]
            pub prevent_fling: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "speed")]
            pub speed: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "gestureSourceType")]
            pub gesture_source_Type: Option<GestureSourceType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "repeatCount")]
            pub repeat_count: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "repeatDelayMs")]
            pub repeat_delay_ms: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "interactionMarkerName")]
            pub interaction_marker_name: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SynthesizeTapGesture {
            #[serde(default)]
            #[serde(rename = "x")]
            pub x: JsFloat,
            #[serde(default)]
            #[serde(rename = "y")]
            pub y: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "duration")]
            pub duration: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "tapCount")]
            pub tap_count: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "gestureSourceType")]
            pub gesture_source_Type: Option<GestureSourceType>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchDragEventReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchKeyEventReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct InsertTextReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ImeSetCompositionReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchMouseEventReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchTouchEventReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EmulateTouchFromMouseEventReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetIgnoreInputEventsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInterceptDragsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SynthesizePinchGestureReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SynthesizeScrollGestureReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SynthesizeTapGestureReturnObject {}
        impl Method for DispatchDragEvent {
            const NAME: &'static str = "Input.dispatchDragEvent";
            type ReturnObject = DispatchDragEventReturnObject;
        }
        impl Method for DispatchKeyEvent {
            const NAME: &'static str = "Input.dispatchKeyEvent";
            type ReturnObject = DispatchKeyEventReturnObject;
        }
        impl Method for InsertText {
            const NAME: &'static str = "Input.insertText";
            type ReturnObject = InsertTextReturnObject;
        }
        impl Method for ImeSetComposition {
            const NAME: &'static str = "Input.imeSetComposition";
            type ReturnObject = ImeSetCompositionReturnObject;
        }
        impl Method for DispatchMouseEvent {
            const NAME: &'static str = "Input.dispatchMouseEvent";
            type ReturnObject = DispatchMouseEventReturnObject;
        }
        impl Method for DispatchTouchEvent {
            const NAME: &'static str = "Input.dispatchTouchEvent";
            type ReturnObject = DispatchTouchEventReturnObject;
        }
        impl Method for EmulateTouchFromMouseEvent {
            const NAME: &'static str = "Input.emulateTouchFromMouseEvent";
            type ReturnObject = EmulateTouchFromMouseEventReturnObject;
        }
        impl Method for SetIgnoreInputEvents {
            const NAME: &'static str = "Input.setIgnoreInputEvents";
            type ReturnObject = SetIgnoreInputEventsReturnObject;
        }
        impl Method for SetInterceptDrags {
            const NAME: &'static str = "Input.setInterceptDrags";
            type ReturnObject = SetInterceptDragsReturnObject;
        }
        impl Method for SynthesizePinchGesture {
            const NAME: &'static str = "Input.synthesizePinchGesture";
            type ReturnObject = SynthesizePinchGestureReturnObject;
        }
        impl Method for SynthesizeScrollGesture {
            const NAME: &'static str = "Input.synthesizeScrollGesture";
            type ReturnObject = SynthesizeScrollGestureReturnObject;
        }
        impl Method for SynthesizeTapGesture {
            const NAME: &'static str = "Input.synthesizeTapGesture";
            type ReturnObject = SynthesizeTapGestureReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DragInterceptedEvent {
                pub params: DragInterceptedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DragInterceptedEventParams {
                #[serde(rename = "data")]
                pub data: super::DragData,
            }
        }
    }
    pub mod Inspector {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        impl Method for Disable {
            const NAME: &'static str = "Inspector.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Inspector.enable";
            type ReturnObject = EnableReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DetachedEvent {
                pub params: DetachedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DetachedEventParams {
                #[serde(default)]
                #[serde(rename = "reason")]
                pub reason: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct TargetCrashedEvent(pub Option<serde_json::Value>);
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct TargetReloadedAfterCrashEvent(pub Option<serde_json::Value>);
        }
    }
    pub mod LayerTree {
        use super::types::*;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type LayerId = String;
        pub type SnapshotId = String;
        pub type PaintProfile = Vec<JsFloat>;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ScrollRectType {
            #[serde(rename = "RepaintsOnScroll")]
            RepaintsOnScroll,
            #[serde(rename = "TouchEventHandler")]
            TouchEventHandler,
            #[serde(rename = "WheelEventHandler")]
            WheelEventHandler,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ScrollRect {
            #[serde(rename = "rect")]
            pub rect: DOM::Rect,
            #[serde(rename = "type")]
            pub Type: ScrollRectType,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StickyPositionConstraint {
            #[serde(rename = "stickyBoxRect")]
            pub sticky_box_rect: DOM::Rect,
            #[serde(rename = "containingBlockRect")]
            pub containing_block_rect: DOM::Rect,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nearestLayerShiftingStickyBox")]
            pub nearest_layer_shifting_sticky_box: Option<LayerId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nearestLayerShiftingContainingBlock")]
            pub nearest_layer_shifting_containing_block: Option<LayerId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PictureTile {
            #[serde(default)]
            #[serde(rename = "x")]
            pub x: JsFloat,
            #[serde(default)]
            #[serde(rename = "y")]
            pub y: JsFloat,
            #[serde(default)]
            #[serde(rename = "picture")]
            pub picture: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Layer {
            #[serde(rename = "layerId")]
            pub layer_id: LayerId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "parentLayerId")]
            pub parent_layer_id: Option<LayerId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
            #[serde(default)]
            #[serde(rename = "offsetX")]
            pub offset_x: JsFloat,
            #[serde(default)]
            #[serde(rename = "offsetY")]
            pub offset_y: JsFloat,
            #[serde(default)]
            #[serde(rename = "width")]
            pub width: JsFloat,
            #[serde(default)]
            #[serde(rename = "height")]
            pub height: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "transform")]
            pub transform: Option<Vec<JsFloat>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "anchorX")]
            pub anchor_x: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "anchorY")]
            pub anchor_y: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "anchorZ")]
            pub anchor_z: Option<JsFloat>,
            #[serde(default)]
            #[serde(rename = "paintCount")]
            pub paint_count: JsUInt,
            #[serde(default)]
            #[serde(rename = "drawsContent")]
            pub draws_content: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "invisible")]
            pub invisible: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "scrollRects")]
            pub scroll_rects: Option<Vec<ScrollRect>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "stickyPositionConstraint")]
            pub sticky_position_constraint: Option<StickyPositionConstraint>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CompositingReasons {
            #[serde(rename = "layerId")]
            pub layer_id: LayerId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct LoadSnapshot {
            #[serde(rename = "tiles")]
            pub tiles: Vec<PictureTile>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct MakeSnapshot {
            #[serde(rename = "layerId")]
            pub layer_id: LayerId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ProfileSnapshot {
            #[serde(rename = "snapshotId")]
            pub snapshot_id: SnapshotId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "minRepeatCount")]
            pub min_repeat_count: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "minDuration")]
            pub min_duration: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "clipRect")]
            pub clip_rect: Option<DOM::Rect>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ReleaseSnapshot {
            #[serde(rename = "snapshotId")]
            pub snapshot_id: SnapshotId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ReplaySnapshot {
            #[serde(rename = "snapshotId")]
            pub snapshot_id: SnapshotId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "fromStep")]
            pub from_step: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "toStep")]
            pub to_step: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "scale")]
            pub scale: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SnapshotCommandLog {
            #[serde(rename = "snapshotId")]
            pub snapshot_id: SnapshotId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CompositingReasonsReturnObject {
            #[serde(rename = "compositingReasons")]
            pub compositing_reasons: Vec<String>,
            #[serde(rename = "compositingReasonIds")]
            pub compositing_reason_ids: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct LoadSnapshotReturnObject {
            #[serde(rename = "snapshotId")]
            pub snapshot_id: SnapshotId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct MakeSnapshotReturnObject {
            #[serde(rename = "snapshotId")]
            pub snapshot_id: SnapshotId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ProfileSnapshotReturnObject {
            #[serde(rename = "timings")]
            pub timings: Vec<PaintProfile>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReleaseSnapshotReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ReplaySnapshotReturnObject {
            #[serde(default)]
            #[serde(rename = "dataURL")]
            pub data_url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SnapshotCommandLogReturnObject {}
        impl Method for CompositingReasons {
            const NAME: &'static str = "LayerTree.compositingReasons";
            type ReturnObject = CompositingReasonsReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "LayerTree.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "LayerTree.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for LoadSnapshot {
            const NAME: &'static str = "LayerTree.loadSnapshot";
            type ReturnObject = LoadSnapshotReturnObject;
        }
        impl Method for MakeSnapshot {
            const NAME: &'static str = "LayerTree.makeSnapshot";
            type ReturnObject = MakeSnapshotReturnObject;
        }
        impl Method for ProfileSnapshot {
            const NAME: &'static str = "LayerTree.profileSnapshot";
            type ReturnObject = ProfileSnapshotReturnObject;
        }
        impl Method for ReleaseSnapshot {
            const NAME: &'static str = "LayerTree.releaseSnapshot";
            type ReturnObject = ReleaseSnapshotReturnObject;
        }
        impl Method for ReplaySnapshot {
            const NAME: &'static str = "LayerTree.replaySnapshot";
            type ReturnObject = ReplaySnapshotReturnObject;
        }
        impl Method for SnapshotCommandLog {
            const NAME: &'static str = "LayerTree.snapshotCommandLog";
            type ReturnObject = SnapshotCommandLogReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LayerPaintedEvent {
                pub params: LayerPaintedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LayerPaintedEventParams {
                #[serde(rename = "layerId")]
                pub layer_id: super::LayerId,
                #[serde(rename = "clip")]
                pub clip: super::super::DOM::Rect,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LayerTreeDidChangeEvent {
                pub params: LayerTreeDidChangeEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LayerTreeDidChangeEventParams {
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "layers")]
                pub layers: Option<Vec<super::Layer>>,
            }
        }
    }
    pub mod Log {
        use super::types::*;
        use super::Network;
        use super::Runtime;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum LogEntrySource {
            #[serde(rename = "xml")]
            Xml,
            #[serde(rename = "javascript")]
            Javascript,
            #[serde(rename = "network")]
            Network,
            #[serde(rename = "storage")]
            Storage,
            #[serde(rename = "appcache")]
            Appcache,
            #[serde(rename = "rendering")]
            Rendering,
            #[serde(rename = "security")]
            Security,
            #[serde(rename = "deprecation")]
            Deprecation,
            #[serde(rename = "worker")]
            Worker,
            #[serde(rename = "violation")]
            Violation,
            #[serde(rename = "intervention")]
            Intervention,
            #[serde(rename = "recommendation")]
            Recommendation,
            #[serde(rename = "other")]
            Other,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum LogEntryLevel {
            #[serde(rename = "verbose")]
            Verbose,
            #[serde(rename = "info")]
            Info,
            #[serde(rename = "warning")]
            Warning,
            #[serde(rename = "error")]
            Error,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum LogEntryCategory {
            #[serde(rename = "cors")]
            Cors,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ViolationSettingName {
            #[serde(rename = "longTask")]
            LongTask,
            #[serde(rename = "longLayout")]
            LongLayout,
            #[serde(rename = "blockedEvent")]
            BlockedEvent,
            #[serde(rename = "blockedParser")]
            BlockedParser,
            #[serde(rename = "discouragedAPIUse")]
            DiscouragedApiUse,
            #[serde(rename = "handler")]
            Handler,
            #[serde(rename = "recurringHandler")]
            RecurringHandler,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct LogEntry {
            #[serde(rename = "source")]
            pub source: LogEntrySource,
            #[serde(rename = "level")]
            pub level: LogEntryLevel,
            #[serde(default)]
            #[serde(rename = "text")]
            pub text: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "category")]
            pub category: Option<LogEntryCategory>,
            #[serde(rename = "timestamp")]
            pub timestamp: Runtime::Timestamp,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "lineNumber")]
            pub line_number: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "stackTrace")]
            pub stack_trace: Option<Runtime::StackTrace>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "networkRequestId")]
            pub network_request_id: Option<Network::RequestId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "workerId")]
            pub worker_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "args")]
            pub args: Option<Vec<Runtime::RemoteObject>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ViolationSetting {
            #[serde(rename = "name")]
            pub name: ViolationSettingName,
            #[serde(default)]
            #[serde(rename = "threshold")]
            pub threshold: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Clear(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StartViolationsReport {
            #[serde(rename = "config")]
            pub config: Vec<ViolationSetting>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopViolationsReport(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartViolationsReportReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopViolationsReportReturnObject {}
        impl Method for Clear {
            const NAME: &'static str = "Log.clear";
            type ReturnObject = ClearReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Log.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Log.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for StartViolationsReport {
            const NAME: &'static str = "Log.startViolationsReport";
            type ReturnObject = StartViolationsReportReturnObject;
        }
        impl Method for StopViolationsReport {
            const NAME: &'static str = "Log.stopViolationsReport";
            type ReturnObject = StopViolationsReportReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct EntryAddedEvent {
                pub params: EntryAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct EntryAddedEventParams {
                #[serde(rename = "entry")]
                pub entry: super::LogEntry,
            }
        }
    }
    pub mod Memory {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum PressureLevel {
            #[serde(rename = "moderate")]
            Moderate,
            #[serde(rename = "critical")]
            Critical,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SamplingProfileNode {
            #[serde(default)]
            #[serde(rename = "size")]
            pub size: JsFloat,
            #[serde(default)]
            #[serde(rename = "total")]
            pub total: JsFloat,
            #[serde(default)]
            #[serde(rename = "stack")]
            pub stack: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SamplingProfile {
            #[serde(rename = "samples")]
            pub samples: Vec<SamplingProfileNode>,
            #[serde(rename = "modules")]
            pub modules: Vec<Module>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Module {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "uuid")]
            pub uuid: String,
            #[serde(default)]
            #[serde(rename = "baseAddress")]
            pub base_address: String,
            #[serde(default)]
            #[serde(rename = "size")]
            pub size: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetDOMCounters(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PrepareForLeakDetection(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ForciblyPurgeJavaScriptMemory(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetPressureNotificationsSuppressed {
            #[serde(default)]
            #[serde(rename = "suppressed")]
            pub suppressed: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SimulatePressureNotification {
            #[serde(rename = "level")]
            pub level: PressureLevel,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StartSampling {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "samplingInterval")]
            pub sampling_interval: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "suppressRandomness")]
            pub suppress_randomness: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopSampling(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetAllTimeSamplingProfile(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetBrowserSamplingProfile(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetSamplingProfile(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetDOMCountersReturnObject {
            #[serde(default)]
            #[serde(rename = "documents")]
            pub documents: JsUInt,
            #[serde(default)]
            #[serde(rename = "nodes")]
            pub nodes: JsUInt,
            #[serde(default)]
            #[serde(rename = "jsEventListeners")]
            pub js_event_listeners: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PrepareForLeakDetectionReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ForciblyPurgeJavaScriptMemoryReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPressureNotificationsSuppressedReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SimulatePressureNotificationReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartSamplingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopSamplingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetAllTimeSamplingProfileReturnObject {
            #[serde(rename = "profile")]
            pub profile: SamplingProfile,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetBrowserSamplingProfileReturnObject {
            #[serde(rename = "profile")]
            pub profile: SamplingProfile,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetSamplingProfileReturnObject {
            #[serde(rename = "profile")]
            pub profile: SamplingProfile,
        }
        impl Method for GetDOMCounters {
            const NAME: &'static str = "Memory.getDOMCounters";
            type ReturnObject = GetDOMCountersReturnObject;
        }
        impl Method for PrepareForLeakDetection {
            const NAME: &'static str = "Memory.prepareForLeakDetection";
            type ReturnObject = PrepareForLeakDetectionReturnObject;
        }
        impl Method for ForciblyPurgeJavaScriptMemory {
            const NAME: &'static str = "Memory.forciblyPurgeJavaScriptMemory";
            type ReturnObject = ForciblyPurgeJavaScriptMemoryReturnObject;
        }
        impl Method for SetPressureNotificationsSuppressed {
            const NAME: &'static str = "Memory.setPressureNotificationsSuppressed";
            type ReturnObject = SetPressureNotificationsSuppressedReturnObject;
        }
        impl Method for SimulatePressureNotification {
            const NAME: &'static str = "Memory.simulatePressureNotification";
            type ReturnObject = SimulatePressureNotificationReturnObject;
        }
        impl Method for StartSampling {
            const NAME: &'static str = "Memory.startSampling";
            type ReturnObject = StartSamplingReturnObject;
        }
        impl Method for StopSampling {
            const NAME: &'static str = "Memory.stopSampling";
            type ReturnObject = StopSamplingReturnObject;
        }
        impl Method for GetAllTimeSamplingProfile {
            const NAME: &'static str = "Memory.getAllTimeSamplingProfile";
            type ReturnObject = GetAllTimeSamplingProfileReturnObject;
        }
        impl Method for GetBrowserSamplingProfile {
            const NAME: &'static str = "Memory.getBrowserSamplingProfile";
            type ReturnObject = GetBrowserSamplingProfileReturnObject;
        }
        impl Method for GetSamplingProfile {
            const NAME: &'static str = "Memory.getSamplingProfile";
            type ReturnObject = GetSamplingProfileReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod Network {
        use super::types::*;
        use super::Debugger;
        use super::Emulation;
        use super::Network;
        use super::Page;
        use super::Runtime;
        use super::Security;
        use super::IO;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type LoaderId = String;
        pub type RequestId = String;
        pub type InterceptionId = String;
        pub type TimeSinceEpoch = JsFloat;
        pub type MonotonicTime = JsFloat;
        pub type ReportId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ResourceType {
            #[serde(rename = "Document")]
            Document,
            #[serde(rename = "Stylesheet")]
            Stylesheet,
            #[serde(rename = "Image")]
            Image,
            #[serde(rename = "Media")]
            Media,
            #[serde(rename = "Font")]
            Font,
            #[serde(rename = "Script")]
            Script,
            #[serde(rename = "TextTrack")]
            TextTrack,
            #[serde(rename = "XHR")]
            Xhr,
            #[serde(rename = "Fetch")]
            Fetch,
            #[serde(rename = "EventSource")]
            EventSource,
            #[serde(rename = "WebSocket")]
            WebSocket,
            #[serde(rename = "Manifest")]
            Manifest,
            #[serde(rename = "SignedExchange")]
            SignedExchange,
            #[serde(rename = "Ping")]
            Ping,
            #[serde(rename = "CSPViolationReport")]
            CspViolationReport,
            #[serde(rename = "Preflight")]
            Preflight,
            #[serde(rename = "Other")]
            Other,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ErrorReason {
            #[serde(rename = "Failed")]
            Failed,
            #[serde(rename = "Aborted")]
            Aborted,
            #[serde(rename = "TimedOut")]
            TimedOut,
            #[serde(rename = "AccessDenied")]
            AccessDenied,
            #[serde(rename = "ConnectionClosed")]
            ConnectionClosed,
            #[serde(rename = "ConnectionReset")]
            ConnectionReset,
            #[serde(rename = "ConnectionRefused")]
            ConnectionRefused,
            #[serde(rename = "ConnectionAborted")]
            ConnectionAborted,
            #[serde(rename = "ConnectionFailed")]
            ConnectionFailed,
            #[serde(rename = "NameNotResolved")]
            NameNotResolved,
            #[serde(rename = "InternetDisconnected")]
            InternetDisconnected,
            #[serde(rename = "AddressUnreachable")]
            AddressUnreachable,
            #[serde(rename = "BlockedByClient")]
            BlockedByClient,
            #[serde(rename = "BlockedByResponse")]
            BlockedByResponse,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ConnectionType {
            #[serde(rename = "none")]
            None,
            #[serde(rename = "cellular2g")]
            Cellular2G,
            #[serde(rename = "cellular3g")]
            Cellular3G,
            #[serde(rename = "cellular4g")]
            Cellular4G,
            #[serde(rename = "bluetooth")]
            Bluetooth,
            #[serde(rename = "ethernet")]
            Ethernet,
            #[serde(rename = "wifi")]
            Wifi,
            #[serde(rename = "wimax")]
            Wimax,
            #[serde(rename = "other")]
            Other,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum CookieSameSite {
            #[serde(rename = "Strict")]
            Strict,
            #[serde(rename = "Lax")]
            Lax,
            #[serde(rename = "None")]
            None,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum CookiePriority {
            #[serde(rename = "Low")]
            Low,
            #[serde(rename = "Medium")]
            Medium,
            #[serde(rename = "High")]
            High,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum CookieSourceScheme {
            #[serde(rename = "Unset")]
            Unset,
            #[serde(rename = "NonSecure")]
            NonSecure,
            #[serde(rename = "Secure")]
            Secure,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ResourcePriority {
            #[serde(rename = "VeryLow")]
            VeryLow,
            #[serde(rename = "Low")]
            Low,
            #[serde(rename = "Medium")]
            Medium,
            #[serde(rename = "High")]
            High,
            #[serde(rename = "VeryHigh")]
            VeryHigh,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum RequestReferrerPolicy {
            #[serde(rename = "unsafe-url")]
            UnsafeUrl,
            #[serde(rename = "no-referrer-when-downgrade")]
            NoReferrerWhenDowngrade,
            #[serde(rename = "no-referrer")]
            NoReferrer,
            #[serde(rename = "origin")]
            Origin,
            #[serde(rename = "origin-when-cross-origin")]
            OriginWhenCrossOrigin,
            #[serde(rename = "same-origin")]
            SameOrigin,
            #[serde(rename = "strict-origin")]
            StrictOrigin,
            #[serde(rename = "strict-origin-when-cross-origin")]
            StrictOriginWhenCrossOrigin,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum CertificateTransparencyCompliance {
            #[serde(rename = "unknown")]
            Unknown,
            #[serde(rename = "not-compliant")]
            NotCompliant,
            #[serde(rename = "compliant")]
            Compliant,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum BlockedReason {
            #[serde(rename = "other")]
            Other,
            #[serde(rename = "csp")]
            Csp,
            #[serde(rename = "mixed-content")]
            MixedContent,
            #[serde(rename = "origin")]
            Origin,
            #[serde(rename = "inspector")]
            Inspector,
            #[serde(rename = "subresource-filter")]
            SubresourceFilter,
            #[serde(rename = "content-type")]
            ContentType,
            #[serde(rename = "coep-frame-resource-needs-coep-header")]
            CoepFrameResourceNeedsCoepHeader,
            #[serde(rename = "coop-sandboxed-iframe-cannot-navigate-to-coop-page")]
            CoopSandboxedIframeCannotNavigateToCoopPage,
            #[serde(rename = "corp-not-same-origin")]
            CorpNotSameOrigin,
            #[serde(rename = "corp-not-same-origin-after-defaulted-to-same-origin-by-coep")]
            CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
            #[serde(rename = "corp-not-same-site")]
            CorpNotSameSite,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum CorsError {
            #[serde(rename = "DisallowedByMode")]
            DisallowedByMode,
            #[serde(rename = "InvalidResponse")]
            InvalidResponse,
            #[serde(rename = "WildcardOriginNotAllowed")]
            WildcardOriginNotAllowed,
            #[serde(rename = "MissingAllowOriginHeader")]
            MissingAllowOriginHeader,
            #[serde(rename = "MultipleAllowOriginValues")]
            MultipleAllowOriginValues,
            #[serde(rename = "InvalidAllowOriginValue")]
            InvalidAllowOriginValue,
            #[serde(rename = "AllowOriginMismatch")]
            AllowOriginMismatch,
            #[serde(rename = "InvalidAllowCredentials")]
            InvalidAllowCredentials,
            #[serde(rename = "CorsDisabledScheme")]
            CorsDisabledScheme,
            #[serde(rename = "PreflightInvalidStatus")]
            PreflightInvalidStatus,
            #[serde(rename = "PreflightDisallowedRedirect")]
            PreflightDisallowedRedirect,
            #[serde(rename = "PreflightWildcardOriginNotAllowed")]
            PreflightWildcardOriginNotAllowed,
            #[serde(rename = "PreflightMissingAllowOriginHeader")]
            PreflightMissingAllowOriginHeader,
            #[serde(rename = "PreflightMultipleAllowOriginValues")]
            PreflightMultipleAllowOriginValues,
            #[serde(rename = "PreflightInvalidAllowOriginValue")]
            PreflightInvalidAllowOriginValue,
            #[serde(rename = "PreflightAllowOriginMismatch")]
            PreflightAllowOriginMismatch,
            #[serde(rename = "PreflightInvalidAllowCredentials")]
            PreflightInvalidAllowCredentials,
            #[serde(rename = "PreflightMissingAllowExternal")]
            PreflightMissingAllowExternal,
            #[serde(rename = "PreflightInvalidAllowExternal")]
            PreflightInvalidAllowExternal,
            #[serde(rename = "InvalidAllowMethodsPreflightResponse")]
            InvalidAllowMethodsPreflightResponse,
            #[serde(rename = "InvalidAllowHeadersPreflightResponse")]
            InvalidAllowHeadersPreflightResponse,
            #[serde(rename = "MethodDisallowedByPreflightResponse")]
            MethodDisallowedByPreflightResponse,
            #[serde(rename = "HeaderDisallowedByPreflightResponse")]
            HeaderDisallowedByPreflightResponse,
            #[serde(rename = "RedirectContainsCredentials")]
            RedirectContainsCredentials,
            #[serde(rename = "InsecurePrivateNetwork")]
            InsecurePrivateNetwork,
            #[serde(rename = "InvalidPrivateNetworkAccess")]
            InvalidPrivateNetworkAccess,
            #[serde(rename = "UnexpectedPrivateNetworkAccess")]
            UnexpectedPrivateNetworkAccess,
            #[serde(rename = "NoCorsRedirectModeNotFollow")]
            NoCorsRedirectModeNotFollow,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ServiceWorkerResponseSource {
            #[serde(rename = "cache-storage")]
            CacheStorage,
            #[serde(rename = "http-cache")]
            HttpCache,
            #[serde(rename = "fallback-code")]
            FallbackCode,
            #[serde(rename = "network")]
            Network,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum TrustTokenParamsRefreshPolicy {
            #[serde(rename = "UseCached")]
            UseCached,
            #[serde(rename = "Refresh")]
            Refresh,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum TrustTokenOperationType {
            #[serde(rename = "Issuance")]
            Issuance,
            #[serde(rename = "Redemption")]
            Redemption,
            #[serde(rename = "Signing")]
            Signing,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum InitiatorType {
            #[serde(rename = "parser")]
            Parser,
            #[serde(rename = "script")]
            Script,
            #[serde(rename = "preload")]
            Preload,
            #[serde(rename = "SignedExchange")]
            SignedExchange,
            #[serde(rename = "preflight")]
            Preflight,
            #[serde(rename = "other")]
            Other,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SetCookieBlockedReason {
            #[serde(rename = "SecureOnly")]
            SecureOnly,
            #[serde(rename = "SameSiteStrict")]
            SameSiteStrict,
            #[serde(rename = "SameSiteLax")]
            SameSiteLax,
            #[serde(rename = "SameSiteUnspecifiedTreatedAsLax")]
            SameSiteUnspecifiedTreatedAsLax,
            #[serde(rename = "SameSiteNoneInsecure")]
            SameSiteNoneInsecure,
            #[serde(rename = "UserPreferences")]
            UserPreferences,
            #[serde(rename = "SyntaxError")]
            SyntaxError,
            #[serde(rename = "SchemeNotSupported")]
            SchemeNotSupported,
            #[serde(rename = "OverwriteSecure")]
            OverwriteSecure,
            #[serde(rename = "InvalidDomain")]
            InvalidDomain,
            #[serde(rename = "InvalidPrefix")]
            InvalidPrefix,
            #[serde(rename = "UnknownError")]
            UnknownError,
            #[serde(rename = "SchemefulSameSiteStrict")]
            SchemefulSameSiteStrict,
            #[serde(rename = "SchemefulSameSiteLax")]
            SchemefulSameSiteLax,
            #[serde(rename = "SchemefulSameSiteUnspecifiedTreatedAsLax")]
            SchemefulSameSiteUnspecifiedTreatedAsLax,
            #[serde(rename = "SamePartyFromCrossPartyContext")]
            SamePartyFromCrossPartyContext,
            #[serde(rename = "SamePartyConflictsWithOtherAttributes")]
            SamePartyConflictsWithOtherAttributes,
            #[serde(rename = "NameValuePairExceedsMaxSize")]
            NameValuePairExceedsMaxSize,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum CookieBlockedReason {
            #[serde(rename = "SecureOnly")]
            SecureOnly,
            #[serde(rename = "NotOnPath")]
            NotOnPath,
            #[serde(rename = "DomainMismatch")]
            DomainMismatch,
            #[serde(rename = "SameSiteStrict")]
            SameSiteStrict,
            #[serde(rename = "SameSiteLax")]
            SameSiteLax,
            #[serde(rename = "SameSiteUnspecifiedTreatedAsLax")]
            SameSiteUnspecifiedTreatedAsLax,
            #[serde(rename = "SameSiteNoneInsecure")]
            SameSiteNoneInsecure,
            #[serde(rename = "UserPreferences")]
            UserPreferences,
            #[serde(rename = "UnknownError")]
            UnknownError,
            #[serde(rename = "SchemefulSameSiteStrict")]
            SchemefulSameSiteStrict,
            #[serde(rename = "SchemefulSameSiteLax")]
            SchemefulSameSiteLax,
            #[serde(rename = "SchemefulSameSiteUnspecifiedTreatedAsLax")]
            SchemefulSameSiteUnspecifiedTreatedAsLax,
            #[serde(rename = "SamePartyFromCrossPartyContext")]
            SamePartyFromCrossPartyContext,
            #[serde(rename = "NameValuePairExceedsMaxSize")]
            NameValuePairExceedsMaxSize,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum AuthChallengeSource {
            #[serde(rename = "Server")]
            Server,
            #[serde(rename = "Proxy")]
            Proxy,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum AuthChallengeResponseResponse {
            #[serde(rename = "Default")]
            Default,
            #[serde(rename = "CancelAuth")]
            CancelAuth,
            #[serde(rename = "ProvideCredentials")]
            ProvideCredentials,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum InterceptionStage {
            #[serde(rename = "Request")]
            Request,
            #[serde(rename = "HeadersReceived")]
            HeadersReceived,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SignedExchangeErrorField {
            #[serde(rename = "signatureSig")]
            SignatureSig,
            #[serde(rename = "signatureIntegrity")]
            SignatureIntegrity,
            #[serde(rename = "signatureCertUrl")]
            SignatureCertUrl,
            #[serde(rename = "signatureCertSha256")]
            SignatureCertSha256,
            #[serde(rename = "signatureValidityUrl")]
            SignatureValidityUrl,
            #[serde(rename = "signatureTimestamps")]
            SignatureTimestamps,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ContentEncoding {
            #[serde(rename = "deflate")]
            Deflate,
            #[serde(rename = "gzip")]
            Gzip,
            #[serde(rename = "br")]
            Br,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum PrivateNetworkRequestPolicy {
            #[serde(rename = "Allow")]
            Allow,
            #[serde(rename = "BlockFromInsecureToMorePrivate")]
            BlockFromInsecureToMorePrivate,
            #[serde(rename = "WarnFromInsecureToMorePrivate")]
            WarnFromInsecureToMorePrivate,
            #[serde(rename = "PreflightBlock")]
            PreflightBlock,
            #[serde(rename = "PreflightWarn")]
            PreflightWarn,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum IPAddressSpace {
            #[serde(rename = "Local")]
            Local,
            #[serde(rename = "Private")]
            Private,
            #[serde(rename = "Public")]
            Public,
            #[serde(rename = "Unknown")]
            Unknown,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum CrossOriginOpenerPolicyValue {
            #[serde(rename = "SameOrigin")]
            SameOrigin,
            #[serde(rename = "SameOriginAllowPopups")]
            SameOriginAllowPopups,
            #[serde(rename = "UnsafeNone")]
            UnsafeNone,
            #[serde(rename = "SameOriginPlusCoep")]
            SameOriginPlusCoep,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum CrossOriginEmbedderPolicyValue {
            #[serde(rename = "None")]
            None,
            #[serde(rename = "Credentialless")]
            Credentialless,
            #[serde(rename = "RequireCorp")]
            RequireCorp,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ReportStatus {
            #[serde(rename = "Queued")]
            Queued,
            #[serde(rename = "Pending")]
            Pending,
            #[serde(rename = "MarkedForRemoval")]
            MarkedForRemoval,
            #[serde(rename = "Success")]
            Success,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum TrustTokenOperationDoneEventStatusOption {
            #[serde(rename = "Ok")]
            Ok,
            #[serde(rename = "InvalidArgument")]
            InvalidArgument,
            #[serde(rename = "FailedPrecondition")]
            FailedPrecondition,
            #[serde(rename = "ResourceExhausted")]
            ResourceExhausted,
            #[serde(rename = "AlreadyExists")]
            AlreadyExists,
            #[serde(rename = "Unavailable")]
            Unavailable,
            #[serde(rename = "BadResponse")]
            BadResponse,
            #[serde(rename = "InternalError")]
            InternalError,
            #[serde(rename = "UnknownError")]
            UnknownError,
            #[serde(rename = "FulfilledLocally")]
            FulfilledLocally,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Headers(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ResourceTiming {
            #[serde(default)]
            #[serde(rename = "requestTime")]
            pub request_time: JsFloat,
            #[serde(default)]
            #[serde(rename = "proxyStart")]
            pub proxy_start: JsFloat,
            #[serde(default)]
            #[serde(rename = "proxyEnd")]
            pub proxy_end: JsFloat,
            #[serde(default)]
            #[serde(rename = "dnsStart")]
            pub dns_start: JsFloat,
            #[serde(default)]
            #[serde(rename = "dnsEnd")]
            pub dns_end: JsFloat,
            #[serde(default)]
            #[serde(rename = "connectStart")]
            pub connect_start: JsFloat,
            #[serde(default)]
            #[serde(rename = "connectEnd")]
            pub connect_end: JsFloat,
            #[serde(default)]
            #[serde(rename = "sslStart")]
            pub ssl_start: JsFloat,
            #[serde(default)]
            #[serde(rename = "sslEnd")]
            pub ssl_end: JsFloat,
            #[serde(default)]
            #[serde(rename = "workerStart")]
            pub worker_start: JsFloat,
            #[serde(default)]
            #[serde(rename = "workerReady")]
            pub worker_ready: JsFloat,
            #[serde(default)]
            #[serde(rename = "workerFetchStart")]
            pub worker_fetch_start: JsFloat,
            #[serde(default)]
            #[serde(rename = "workerRespondWithSettled")]
            pub worker_respond_with_settled: JsFloat,
            #[serde(default)]
            #[serde(rename = "sendStart")]
            pub send_start: JsFloat,
            #[serde(default)]
            #[serde(rename = "sendEnd")]
            pub send_end: JsFloat,
            #[serde(default)]
            #[serde(rename = "pushStart")]
            pub push_start: JsFloat,
            #[serde(default)]
            #[serde(rename = "pushEnd")]
            pub push_end: JsFloat,
            #[serde(default)]
            #[serde(rename = "receiveHeadersEnd")]
            pub receive_headers_end: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PostDataEntry {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "bytes")]
            pub bytes: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Request {
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "urlFragment")]
            pub url_fragment: Option<String>,
            #[serde(default)]
            #[serde(rename = "method")]
            pub method: String,
            #[serde(rename = "headers")]
            pub headers: Headers,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "postData")]
            pub post_data: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "hasPostData")]
            pub has_post_data: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "postDataEntries")]
            pub post_data_entries: Option<Vec<PostDataEntry>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "mixedContentType")]
            pub mixed_content_Type: Option<Security::MixedContentType>,
            #[serde(rename = "initialPriority")]
            pub initial_priority: ResourcePriority,
            #[serde(rename = "referrerPolicy")]
            pub referrer_policy: RequestReferrerPolicy,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "isLinkPreload")]
            pub is_link_preload: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "trustTokenParams")]
            pub trust_token_params: Option<TrustTokenParams>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "isSameSite")]
            pub is_same_site: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SignedCertificateTimestamp {
            #[serde(default)]
            #[serde(rename = "status")]
            pub status: String,
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: String,
            #[serde(default)]
            #[serde(rename = "logDescription")]
            pub log_description: String,
            #[serde(default)]
            #[serde(rename = "logId")]
            pub log_id: String,
            #[serde(default)]
            #[serde(rename = "timestamp")]
            pub timestamp: JsFloat,
            #[serde(default)]
            #[serde(rename = "hashAlgorithm")]
            pub hash_algorithm: String,
            #[serde(default)]
            #[serde(rename = "signatureAlgorithm")]
            pub signature_algorithm: String,
            #[serde(default)]
            #[serde(rename = "signatureData")]
            pub signature_data: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SecurityDetails {
            #[serde(default)]
            #[serde(rename = "protocol")]
            pub protocol: String,
            #[serde(default)]
            #[serde(rename = "keyExchange")]
            pub key_exchange: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "keyExchangeGroup")]
            pub key_exchange_group: Option<String>,
            #[serde(default)]
            #[serde(rename = "cipher")]
            pub cipher: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "mac")]
            pub mac: Option<String>,
            #[serde(rename = "certificateId")]
            pub certificate_id: Security::CertificateId,
            #[serde(default)]
            #[serde(rename = "subjectName")]
            pub subject_name: String,
            #[serde(default)]
            #[serde(rename = "sanList")]
            pub san_list: Vec<String>,
            #[serde(default)]
            #[serde(rename = "issuer")]
            pub issuer: String,
            #[serde(rename = "validFrom")]
            pub valid_from: TimeSinceEpoch,
            #[serde(rename = "validTo")]
            pub valid_to: TimeSinceEpoch,
            #[serde(rename = "signedCertificateTimestampList")]
            pub signed_certificate_timestamp_list: Vec<SignedCertificateTimestamp>,
            #[serde(rename = "certificateTransparencyCompliance")]
            pub certificate_transparency_compliance: CertificateTransparencyCompliance,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CorsErrorStatus {
            #[serde(rename = "corsError")]
            pub cors_error: CorsError,
            #[serde(default)]
            #[serde(rename = "failedParameter")]
            pub failed_parameter: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TrustTokenParams {
            #[serde(rename = "type")]
            pub Type: TrustTokenOperationType,
            #[serde(rename = "refreshPolicy")]
            pub refresh_policy: TrustTokenParamsRefreshPolicy,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "issuers")]
            pub issuers: Option<Vec<String>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Response {
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(default)]
            #[serde(rename = "status")]
            pub status: JsUInt,
            #[serde(default)]
            #[serde(rename = "statusText")]
            pub status_text: String,
            #[serde(rename = "headers")]
            pub headers: Headers,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "headersText")]
            pub headers_text: Option<String>,
            #[serde(default)]
            #[serde(rename = "mimeType")]
            pub mime_type: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "requestHeaders")]
            pub request_headers: Option<Headers>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "requestHeadersText")]
            pub request_headers_text: Option<String>,
            #[serde(default)]
            #[serde(rename = "connectionReused")]
            pub connection_reused: bool,
            #[serde(default)]
            #[serde(rename = "connectionId")]
            pub connection_id: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "remoteIPAddress")]
            pub remote_ip_address: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "remotePort")]
            pub remote_port: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "fromDiskCache")]
            pub from_disk_cache: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "fromServiceWorker")]
            pub from_service_worker: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "fromPrefetchCache")]
            pub from_prefetch_cache: Option<bool>,
            #[serde(default)]
            #[serde(rename = "encodedDataLength")]
            pub encoded_data_length: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "timing")]
            pub timing: Option<ResourceTiming>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "serviceWorkerResponseSource")]
            pub service_worker_response_source: Option<ServiceWorkerResponseSource>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "responseTime")]
            pub response_time: Option<TimeSinceEpoch>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "cacheStorageCacheName")]
            pub cache_storage_cache_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "protocol")]
            pub protocol: Option<String>,
            #[serde(rename = "securityState")]
            pub security_state: Security::SecurityState,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "securityDetails")]
            pub security_details: Option<SecurityDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct WebSocketRequest {
            #[serde(rename = "headers")]
            pub headers: Headers,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct WebSocketResponse {
            #[serde(default)]
            #[serde(rename = "status")]
            pub status: JsUInt,
            #[serde(default)]
            #[serde(rename = "statusText")]
            pub status_text: String,
            #[serde(rename = "headers")]
            pub headers: Headers,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "headersText")]
            pub headers_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "requestHeaders")]
            pub request_headers: Option<Headers>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "requestHeadersText")]
            pub request_headers_text: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct WebSocketFrame {
            #[serde(default)]
            #[serde(rename = "opcode")]
            pub opcode: JsFloat,
            #[serde(default)]
            #[serde(rename = "mask")]
            pub mask: bool,
            #[serde(default)]
            #[serde(rename = "payloadData")]
            pub payload_data: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CachedResource {
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(rename = "type")]
            pub Type: ResourceType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "response")]
            pub response: Option<Response>,
            #[serde(default)]
            #[serde(rename = "bodySize")]
            pub body_size: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Initiator {
            #[serde(rename = "type")]
            pub Type: InitiatorType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "stack")]
            pub stack: Option<Runtime::StackTrace>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "lineNumber")]
            pub line_number: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "columnNumber")]
            pub column_number: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "requestId")]
            pub request_id: Option<RequestId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Cookie {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
            #[serde(default)]
            #[serde(rename = "domain")]
            pub domain: String,
            #[serde(default)]
            #[serde(rename = "path")]
            pub path: String,
            #[serde(default)]
            #[serde(rename = "expires")]
            pub expires: JsFloat,
            #[serde(default)]
            #[serde(rename = "size")]
            pub size: JsUInt,
            #[serde(default)]
            #[serde(rename = "httpOnly")]
            pub http_only: bool,
            #[serde(default)]
            #[serde(rename = "secure")]
            pub secure: bool,
            #[serde(default)]
            #[serde(rename = "session")]
            pub session: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "sameSite")]
            pub same_site: Option<CookieSameSite>,
            #[serde(rename = "priority")]
            pub priority: CookiePriority,
            #[serde(default)]
            #[serde(rename = "sameParty")]
            pub same_party: bool,
            #[serde(rename = "sourceScheme")]
            pub source_scheme: CookieSourceScheme,
            #[serde(default)]
            #[serde(rename = "sourcePort")]
            pub source_port: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "partitionKey")]
            pub partition_key: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "partitionKeyOpaque")]
            pub partition_key_opaque: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct BlockedSetCookieWithReason {
            #[serde(rename = "blockedReasons")]
            pub blocked_reasons: Vec<SetCookieBlockedReason>,
            #[serde(default)]
            #[serde(rename = "cookieLine")]
            pub cookie_line: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "cookie")]
            pub cookie: Option<Cookie>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct BlockedCookieWithReason {
            #[serde(rename = "blockedReasons")]
            pub blocked_reasons: Vec<CookieBlockedReason>,
            #[serde(rename = "cookie")]
            pub cookie: Cookie,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CookieParam {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "domain")]
            pub domain: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "path")]
            pub path: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "secure")]
            pub secure: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "httpOnly")]
            pub http_only: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "sameSite")]
            pub same_site: Option<CookieSameSite>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "expires")]
            pub expires: Option<TimeSinceEpoch>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "priority")]
            pub priority: Option<CookiePriority>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "sameParty")]
            pub same_party: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "sourceScheme")]
            pub source_scheme: Option<CookieSourceScheme>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "sourcePort")]
            pub source_port: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "partitionKey")]
            pub partition_key: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AuthChallenge {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "source")]
            pub source: Option<AuthChallengeSource>,
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: String,
            #[serde(default)]
            #[serde(rename = "scheme")]
            pub scheme: String,
            #[serde(default)]
            #[serde(rename = "realm")]
            pub realm: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AuthChallengeResponse {
            #[serde(rename = "response")]
            pub response: AuthChallengeResponseResponse,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "username")]
            pub username: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "password")]
            pub password: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestPattern {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "urlPattern")]
            pub url_pattern: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "resourceType")]
            pub resource_Type: Option<ResourceType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "interceptionStage")]
            pub interception_stage: Option<InterceptionStage>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SignedExchangeSignature {
            #[serde(default)]
            #[serde(rename = "label")]
            pub label: String,
            #[serde(default)]
            #[serde(rename = "signature")]
            pub signature: String,
            #[serde(default)]
            #[serde(rename = "integrity")]
            pub integrity: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "certUrl")]
            pub cert_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "certSha256")]
            pub cert_sha_256: Option<String>,
            #[serde(default)]
            #[serde(rename = "validityUrl")]
            pub validity_url: String,
            #[serde(default)]
            #[serde(rename = "date")]
            pub date: JsUInt,
            #[serde(default)]
            #[serde(rename = "expires")]
            pub expires: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "certificates")]
            pub certificates: Option<Vec<String>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SignedExchangeHeader {
            #[serde(default)]
            #[serde(rename = "requestUrl")]
            pub request_url: String,
            #[serde(default)]
            #[serde(rename = "responseCode")]
            pub response_code: JsUInt,
            #[serde(rename = "responseHeaders")]
            pub response_headers: Headers,
            #[serde(rename = "signatures")]
            pub signatures: Vec<SignedExchangeSignature>,
            #[serde(default)]
            #[serde(rename = "headerIntegrity")]
            pub header_integrity: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SignedExchangeError {
            #[serde(default)]
            #[serde(rename = "message")]
            pub message: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "signatureIndex")]
            pub signature_index: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "errorField")]
            pub error_field: Option<SignedExchangeErrorField>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SignedExchangeInfo {
            #[serde(rename = "outerResponse")]
            pub outer_response: Response,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "header")]
            pub header: Option<SignedExchangeHeader>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "securityDetails")]
            pub security_details: Option<SecurityDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "errors")]
            pub errors: Option<Vec<SignedExchangeError>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ConnectTiming {
            #[serde(default)]
            #[serde(rename = "requestTime")]
            pub request_time: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ClientSecurityState {
            #[serde(default)]
            #[serde(rename = "initiatorIsSecureContext")]
            pub initiator_is_secure_context: bool,
            #[serde(rename = "initiatorIPAddressSpace")]
            pub initiator_ip_address_space: IPAddressSpace,
            #[serde(rename = "privateNetworkRequestPolicy")]
            pub private_network_request_policy: PrivateNetworkRequestPolicy,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CrossOriginOpenerPolicyStatus {
            #[serde(rename = "value")]
            pub value: CrossOriginOpenerPolicyValue,
            #[serde(rename = "reportOnlyValue")]
            pub report_only_value: CrossOriginOpenerPolicyValue,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "reportingEndpoint")]
            pub reporting_endpoint: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "reportOnlyReportingEndpoint")]
            pub report_only_reporting_endpoint: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CrossOriginEmbedderPolicyStatus {
            #[serde(rename = "value")]
            pub value: CrossOriginEmbedderPolicyValue,
            #[serde(rename = "reportOnlyValue")]
            pub report_only_value: CrossOriginEmbedderPolicyValue,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "reportingEndpoint")]
            pub reporting_endpoint: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "reportOnlyReportingEndpoint")]
            pub report_only_reporting_endpoint: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SecurityIsolationStatus {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "coop")]
            pub coop: Option<CrossOriginOpenerPolicyStatus>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "coep")]
            pub coep: Option<CrossOriginEmbedderPolicyStatus>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ReportingApiReport {
            #[serde(rename = "id")]
            pub id: ReportId,
            #[serde(default)]
            #[serde(rename = "initiatorUrl")]
            pub initiator_url: String,
            #[serde(default)]
            #[serde(rename = "destination")]
            pub destination: String,
            #[serde(default)]
            #[serde(rename = "type")]
            pub Type: String,
            #[serde(rename = "timestamp")]
            pub timestamp: Network::TimeSinceEpoch,
            #[serde(default)]
            #[serde(rename = "depth")]
            pub depth: JsUInt,
            #[serde(default)]
            #[serde(rename = "completedAttempts")]
            pub completed_attempts: JsUInt,
            #[serde(rename = "status")]
            pub status: ReportStatus,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ReportingApiEndpoint {
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(default)]
            #[serde(rename = "groupName")]
            pub group_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct LoadNetworkResourcePageResult {
            #[serde(default)]
            #[serde(rename = "success")]
            pub success: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "netError")]
            pub net_error: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "netErrorName")]
            pub net_error_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "httpStatusCode")]
            pub http_status_code: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "stream")]
            pub stream: Option<IO::StreamHandle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "headers")]
            pub headers: Option<Network::Headers>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct LoadNetworkResourceOptions {
            #[serde(default)]
            #[serde(rename = "disableCache")]
            pub disable_cache: bool,
            #[serde(default)]
            #[serde(rename = "includeCredentials")]
            pub include_credentials: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetAcceptedEncodings {
            #[serde(rename = "encodings")]
            pub encodings: Vec<ContentEncoding>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearAcceptedEncodingsOverride(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CanClearBrowserCache(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CanClearBrowserCookies(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CanEmulateNetworkConditions(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearBrowserCache(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearBrowserCookies(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ContinueInterceptedRequest {
            #[serde(rename = "interceptionId")]
            pub interception_id: InterceptionId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "errorReason")]
            pub error_reason: Option<ErrorReason>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "rawResponse")]
            pub raw_response: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "method")]
            pub method: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "postData")]
            pub post_data: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "headers")]
            pub headers: Option<Headers>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "authChallengeResponse")]
            pub auth_challenge_response: Option<AuthChallengeResponse>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DeleteCookies {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "domain")]
            pub domain: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "path")]
            pub path: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct EmulateNetworkConditions {
            #[serde(default)]
            #[serde(rename = "offline")]
            pub offline: bool,
            #[serde(default)]
            #[serde(rename = "latency")]
            pub latency: JsFloat,
            #[serde(default)]
            #[serde(rename = "downloadThroughput")]
            pub download_throughput: JsFloat,
            #[serde(default)]
            #[serde(rename = "uploadThroughput")]
            pub upload_throughput: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "connectionType")]
            pub connection_Type: Option<ConnectionType>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Enable {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "maxTotalBufferSize")]
            pub max_total_buffer_size: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "maxResourceBufferSize")]
            pub max_resource_buffer_size: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "maxPostDataSize")]
            pub max_post_data_size: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetAllCookies(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetCertificate {
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetCookies {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "urls")]
            pub urls: Option<Vec<String>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetResponseBody {
            #[serde(rename = "requestId")]
            pub request_id: RequestId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetRequestPostData {
            #[serde(rename = "requestId")]
            pub request_id: RequestId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetResponseBodyForInterception {
            #[serde(rename = "interceptionId")]
            pub interception_id: InterceptionId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TakeResponseBodyForInterceptionAsStream {
            #[serde(rename = "interceptionId")]
            pub interception_id: InterceptionId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ReplayXHR {
            #[serde(rename = "requestId")]
            pub request_id: RequestId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SearchInResponseBody {
            #[serde(rename = "requestId")]
            pub request_id: RequestId,
            #[serde(default)]
            #[serde(rename = "query")]
            pub query: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "caseSensitive")]
            pub case_sensitive: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "isRegex")]
            pub is_regex: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetBlockedURLs {
            #[serde(default)]
            #[serde(rename = "urls")]
            pub urls: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetBypassServiceWorker {
            #[serde(default)]
            #[serde(rename = "bypass")]
            pub bypass: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetCacheDisabled {
            #[serde(default)]
            #[serde(rename = "cacheDisabled")]
            pub cache_disabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetCookie {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "domain")]
            pub domain: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "path")]
            pub path: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "secure")]
            pub secure: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "httpOnly")]
            pub http_only: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "sameSite")]
            pub same_site: Option<CookieSameSite>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "expires")]
            pub expires: Option<TimeSinceEpoch>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "priority")]
            pub priority: Option<CookiePriority>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "sameParty")]
            pub same_party: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "sourceScheme")]
            pub source_scheme: Option<CookieSourceScheme>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "sourcePort")]
            pub source_port: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "partitionKey")]
            pub partition_key: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetCookies {
            #[serde(rename = "cookies")]
            pub cookies: Vec<CookieParam>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetExtraHTTPHeaders {
            #[serde(rename = "headers")]
            pub headers: Headers,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetAttachDebugStack {
            #[serde(default)]
            #[serde(rename = "enabled")]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetRequestInterception {
            #[serde(rename = "patterns")]
            pub patterns: Vec<RequestPattern>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetUserAgentOverride {
            #[serde(default)]
            #[serde(rename = "userAgent")]
            pub user_agent: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "acceptLanguage")]
            pub accept_language: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "platform")]
            pub platform: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "userAgentMetadata")]
            pub user_agent_metadata: Option<Emulation::UserAgentMetadata>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetSecurityIsolationStatus {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "frameId")]
            pub frame_id: Option<Page::FrameId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct EnableReportingApi {
            #[serde(default)]
            #[serde(rename = "enable")]
            pub enable: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct LoadNetworkResource {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "frameId")]
            pub frame_id: Option<Page::FrameId>,
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(rename = "options")]
            pub options: LoadNetworkResourceOptions,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAcceptedEncodingsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearAcceptedEncodingsOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CanClearBrowserCacheReturnObject {
            #[serde(default)]
            #[serde(rename = "result")]
            pub result: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CanClearBrowserCookiesReturnObject {
            #[serde(default)]
            #[serde(rename = "result")]
            pub result: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CanEmulateNetworkConditionsReturnObject {
            #[serde(default)]
            #[serde(rename = "result")]
            pub result: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearBrowserCacheReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearBrowserCookiesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContinueInterceptedRequestReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteCookiesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EmulateNetworkConditionsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetAllCookiesReturnObject {
            #[serde(rename = "cookies")]
            pub cookies: Vec<Cookie>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetCertificateReturnObject {
            #[serde(rename = "tableNames")]
            pub table_names: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetCookiesReturnObject {
            #[serde(rename = "cookies")]
            pub cookies: Vec<Cookie>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetResponseBodyReturnObject {
            #[serde(default)]
            #[serde(rename = "body")]
            pub body: String,
            #[serde(default)]
            #[serde(rename = "base64Encoded")]
            pub base_64_encoded: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetRequestPostDataReturnObject {
            #[serde(default)]
            #[serde(rename = "postData")]
            pub post_data: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetResponseBodyForInterceptionReturnObject {
            #[serde(default)]
            #[serde(rename = "body")]
            pub body: String,
            #[serde(default)]
            #[serde(rename = "base64Encoded")]
            pub base_64_encoded: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TakeResponseBodyForInterceptionAsStreamReturnObject {
            #[serde(rename = "stream")]
            pub stream: IO::StreamHandle,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReplayXHRReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SearchInResponseBodyReturnObject {
            #[serde(rename = "result")]
            pub result: Debugger::SearchMatch,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBlockedURLsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBypassServiceWorkerReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetCacheDisabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetCookieReturnObject {
            #[serde(default)]
            #[serde(rename = "success")]
            pub success: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetCookiesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetExtraHTTPHeadersReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAttachDebugStackReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetRequestInterceptionReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetUserAgentOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetSecurityIsolationStatusReturnObject {
            #[serde(rename = "status")]
            pub status: SecurityIsolationStatus,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReportingApiReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct LoadNetworkResourceReturnObject {
            #[serde(rename = "resource")]
            pub resource: LoadNetworkResourcePageResult,
        }
        impl Method for SetAcceptedEncodings {
            const NAME: &'static str = "Network.setAcceptedEncodings";
            type ReturnObject = SetAcceptedEncodingsReturnObject;
        }
        impl Method for ClearAcceptedEncodingsOverride {
            const NAME: &'static str = "Network.clearAcceptedEncodingsOverride";
            type ReturnObject = ClearAcceptedEncodingsOverrideReturnObject;
        }
        impl Method for CanClearBrowserCache {
            const NAME: &'static str = "Network.canClearBrowserCache";
            type ReturnObject = CanClearBrowserCacheReturnObject;
        }
        impl Method for CanClearBrowserCookies {
            const NAME: &'static str = "Network.canClearBrowserCookies";
            type ReturnObject = CanClearBrowserCookiesReturnObject;
        }
        impl Method for CanEmulateNetworkConditions {
            const NAME: &'static str = "Network.canEmulateNetworkConditions";
            type ReturnObject = CanEmulateNetworkConditionsReturnObject;
        }
        impl Method for ClearBrowserCache {
            const NAME: &'static str = "Network.clearBrowserCache";
            type ReturnObject = ClearBrowserCacheReturnObject;
        }
        impl Method for ClearBrowserCookies {
            const NAME: &'static str = "Network.clearBrowserCookies";
            type ReturnObject = ClearBrowserCookiesReturnObject;
        }
        impl Method for ContinueInterceptedRequest {
            const NAME: &'static str = "Network.continueInterceptedRequest";
            type ReturnObject = ContinueInterceptedRequestReturnObject;
        }
        impl Method for DeleteCookies {
            const NAME: &'static str = "Network.deleteCookies";
            type ReturnObject = DeleteCookiesReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Network.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for EmulateNetworkConditions {
            const NAME: &'static str = "Network.emulateNetworkConditions";
            type ReturnObject = EmulateNetworkConditionsReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Network.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetAllCookies {
            const NAME: &'static str = "Network.getAllCookies";
            type ReturnObject = GetAllCookiesReturnObject;
        }
        impl Method for GetCertificate {
            const NAME: &'static str = "Network.getCertificate";
            type ReturnObject = GetCertificateReturnObject;
        }
        impl Method for GetCookies {
            const NAME: &'static str = "Network.getCookies";
            type ReturnObject = GetCookiesReturnObject;
        }
        impl Method for GetResponseBody {
            const NAME: &'static str = "Network.getResponseBody";
            type ReturnObject = GetResponseBodyReturnObject;
        }
        impl Method for GetRequestPostData {
            const NAME: &'static str = "Network.getRequestPostData";
            type ReturnObject = GetRequestPostDataReturnObject;
        }
        impl Method for GetResponseBodyForInterception {
            const NAME: &'static str = "Network.getResponseBodyForInterception";
            type ReturnObject = GetResponseBodyForInterceptionReturnObject;
        }
        impl Method for TakeResponseBodyForInterceptionAsStream {
            const NAME: &'static str = "Network.takeResponseBodyForInterceptionAsStream";
            type ReturnObject = TakeResponseBodyForInterceptionAsStreamReturnObject;
        }
        impl Method for ReplayXHR {
            const NAME: &'static str = "Network.replayXHR";
            type ReturnObject = ReplayXHRReturnObject;
        }
        impl Method for SearchInResponseBody {
            const NAME: &'static str = "Network.searchInResponseBody";
            type ReturnObject = SearchInResponseBodyReturnObject;
        }
        impl Method for SetBlockedURLs {
            const NAME: &'static str = "Network.setBlockedURLs";
            type ReturnObject = SetBlockedURLsReturnObject;
        }
        impl Method for SetBypassServiceWorker {
            const NAME: &'static str = "Network.setBypassServiceWorker";
            type ReturnObject = SetBypassServiceWorkerReturnObject;
        }
        impl Method for SetCacheDisabled {
            const NAME: &'static str = "Network.setCacheDisabled";
            type ReturnObject = SetCacheDisabledReturnObject;
        }
        impl Method for SetCookie {
            const NAME: &'static str = "Network.setCookie";
            type ReturnObject = SetCookieReturnObject;
        }
        impl Method for SetCookies {
            const NAME: &'static str = "Network.setCookies";
            type ReturnObject = SetCookiesReturnObject;
        }
        impl Method for SetExtraHTTPHeaders {
            const NAME: &'static str = "Network.setExtraHTTPHeaders";
            type ReturnObject = SetExtraHTTPHeadersReturnObject;
        }
        impl Method for SetAttachDebugStack {
            const NAME: &'static str = "Network.setAttachDebugStack";
            type ReturnObject = SetAttachDebugStackReturnObject;
        }
        impl Method for SetRequestInterception {
            const NAME: &'static str = "Network.setRequestInterception";
            type ReturnObject = SetRequestInterceptionReturnObject;
        }
        impl Method for SetUserAgentOverride {
            const NAME: &'static str = "Network.setUserAgentOverride";
            type ReturnObject = SetUserAgentOverrideReturnObject;
        }
        impl Method for GetSecurityIsolationStatus {
            const NAME: &'static str = "Network.getSecurityIsolationStatus";
            type ReturnObject = GetSecurityIsolationStatusReturnObject;
        }
        impl Method for EnableReportingApi {
            const NAME: &'static str = "Network.enableReportingApi";
            type ReturnObject = EnableReportingApiReturnObject;
        }
        impl Method for LoadNetworkResource {
            const NAME: &'static str = "Network.loadNetworkResource";
            type ReturnObject = LoadNetworkResourceReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DataReceivedEvent {
                pub params: DataReceivedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DataReceivedEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "timestamp")]
                pub timestamp: super::MonotonicTime,
                #[serde(default)]
                #[serde(rename = "dataLength")]
                pub data_length: JsUInt,
                #[serde(default)]
                #[serde(rename = "encodedDataLength")]
                pub encoded_data_length: JsUInt,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct EventSourceMessageReceivedEvent {
                pub params: EventSourceMessageReceivedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct EventSourceMessageReceivedEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "timestamp")]
                pub timestamp: super::MonotonicTime,
                #[serde(default)]
                #[serde(rename = "eventName")]
                pub event_name: String,
                #[serde(default)]
                #[serde(rename = "eventId")]
                pub event_id: String,
                #[serde(default)]
                #[serde(rename = "data")]
                pub data: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LoadingFailedEvent {
                pub params: LoadingFailedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LoadingFailedEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "timestamp")]
                pub timestamp: super::MonotonicTime,
                #[serde(rename = "type")]
                pub Type: super::ResourceType,
                #[serde(default)]
                #[serde(rename = "errorText")]
                pub error_text: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "canceled")]
                pub canceled: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "blockedReason")]
                pub blocked_reason: Option<super::BlockedReason>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "corsErrorStatus")]
                pub cors_error_status: Option<super::CorsErrorStatus>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LoadingFinishedEvent {
                pub params: LoadingFinishedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LoadingFinishedEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "timestamp")]
                pub timestamp: super::MonotonicTime,
                #[serde(default)]
                #[serde(rename = "encodedDataLength")]
                pub encoded_data_length: JsFloat,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "shouldReportCorbBlocking")]
                pub should_report_corb_blocking: Option<bool>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RequestInterceptedEvent {
                pub params: RequestInterceptedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RequestInterceptedEventParams {
                #[serde(rename = "interceptionId")]
                pub interception_id: super::InterceptionId,
                #[serde(rename = "request")]
                pub request: super::Request,
                #[serde(rename = "frameId")]
                pub frame_id: super::super::Page::FrameId,
                #[serde(rename = "resourceType")]
                pub resource_Type: super::ResourceType,
                #[serde(default)]
                #[serde(rename = "isNavigationRequest")]
                pub is_navigation_request: bool,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "isDownload")]
                pub is_download: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "redirectUrl")]
                pub redirect_url: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "authChallenge")]
                pub auth_challenge: Option<super::AuthChallenge>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "responseErrorReason")]
                pub response_error_reason: Option<super::ErrorReason>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "responseStatusCode")]
                pub response_status_code: Option<JsUInt>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "responseHeaders")]
                pub response_headers: Option<super::Headers>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "requestId")]
                pub request_id: Option<super::RequestId>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RequestServedFromCacheEvent {
                pub params: RequestServedFromCacheEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RequestServedFromCacheEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RequestWillBeSentEvent {
                pub params: RequestWillBeSentEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RequestWillBeSentEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "loaderId")]
                pub loader_id: super::LoaderId,
                #[serde(default)]
                #[serde(rename = "documentURL")]
                pub document_url: String,
                #[serde(rename = "request")]
                pub request: super::Request,
                #[serde(rename = "timestamp")]
                pub timestamp: super::MonotonicTime,
                #[serde(rename = "wallTime")]
                pub wall_time: super::TimeSinceEpoch,
                #[serde(rename = "initiator")]
                pub initiator: super::Initiator,
                #[serde(default)]
                #[serde(rename = "redirectHasExtraInfo")]
                pub redirect_has_extra_info: bool,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "redirectResponse")]
                pub redirect_response: Option<super::Response>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "type")]
                pub Type: Option<super::ResourceType>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "frameId")]
                pub frame_id: Option<super::super::Page::FrameId>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "hasUserGesture")]
                pub has_user_gesture: Option<bool>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ResourceChangedPriorityEvent {
                pub params: ResourceChangedPriorityEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ResourceChangedPriorityEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "newPriority")]
                pub new_priority: super::ResourcePriority,
                #[serde(rename = "timestamp")]
                pub timestamp: super::MonotonicTime,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SignedExchangeReceivedEvent {
                pub params: SignedExchangeReceivedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SignedExchangeReceivedEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "info")]
                pub info: super::SignedExchangeInfo,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ResponseReceivedEvent {
                pub params: ResponseReceivedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ResponseReceivedEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "loaderId")]
                pub loader_id: super::LoaderId,
                #[serde(rename = "timestamp")]
                pub timestamp: super::MonotonicTime,
                #[serde(rename = "type")]
                pub Type: super::ResourceType,
                #[serde(rename = "response")]
                pub response: super::Response,
                #[serde(default)]
                #[serde(rename = "hasExtraInfo")]
                pub has_extra_info: bool,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "frameId")]
                pub frame_id: Option<super::super::Page::FrameId>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketClosedEvent {
                pub params: WebSocketClosedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketClosedEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "timestamp")]
                pub timestamp: super::MonotonicTime,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketCreatedEvent {
                pub params: WebSocketCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketCreatedEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(default)]
                #[serde(rename = "url")]
                pub url: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "initiator")]
                pub initiator: Option<super::Initiator>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketFrameErrorEvent {
                pub params: WebSocketFrameErrorEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketFrameErrorEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "timestamp")]
                pub timestamp: super::MonotonicTime,
                #[serde(default)]
                #[serde(rename = "errorMessage")]
                pub error_message: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketFrameReceivedEvent {
                pub params: WebSocketFrameReceivedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketFrameReceivedEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "timestamp")]
                pub timestamp: super::MonotonicTime,
                #[serde(rename = "response")]
                pub response: super::WebSocketFrame,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketFrameSentEvent {
                pub params: WebSocketFrameSentEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketFrameSentEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "timestamp")]
                pub timestamp: super::MonotonicTime,
                #[serde(rename = "response")]
                pub response: super::WebSocketFrame,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketHandshakeResponseReceivedEvent {
                pub params: WebSocketHandshakeResponseReceivedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketHandshakeResponseReceivedEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "timestamp")]
                pub timestamp: super::MonotonicTime,
                #[serde(rename = "response")]
                pub response: super::WebSocketResponse,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketWillSendHandshakeRequestEvent {
                pub params: WebSocketWillSendHandshakeRequestEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketWillSendHandshakeRequestEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "timestamp")]
                pub timestamp: super::MonotonicTime,
                #[serde(rename = "wallTime")]
                pub wall_time: super::TimeSinceEpoch,
                #[serde(rename = "request")]
                pub request: super::WebSocketRequest,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebTransportCreatedEvent {
                pub params: WebTransportCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebTransportCreatedEventParams {
                #[serde(rename = "transportId")]
                pub transport_id: super::RequestId,
                #[serde(default)]
                #[serde(rename = "url")]
                pub url: String,
                #[serde(rename = "timestamp")]
                pub timestamp: super::MonotonicTime,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "initiator")]
                pub initiator: Option<super::Initiator>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebTransportConnectionEstablishedEvent {
                pub params: WebTransportConnectionEstablishedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebTransportConnectionEstablishedEventParams {
                #[serde(rename = "transportId")]
                pub transport_id: super::RequestId,
                #[serde(rename = "timestamp")]
                pub timestamp: super::MonotonicTime,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebTransportClosedEvent {
                pub params: WebTransportClosedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebTransportClosedEventParams {
                #[serde(rename = "transportId")]
                pub transport_id: super::RequestId,
                #[serde(rename = "timestamp")]
                pub timestamp: super::MonotonicTime,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RequestWillBeSentExtraInfoEvent {
                pub params: RequestWillBeSentExtraInfoEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RequestWillBeSentExtraInfoEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "associatedCookies")]
                pub associated_cookies: Vec<super::BlockedCookieWithReason>,
                #[serde(rename = "headers")]
                pub headers: super::Headers,
                #[serde(rename = "connectTiming")]
                pub connect_timing: super::ConnectTiming,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "clientSecurityState")]
                pub client_security_state: Option<super::ClientSecurityState>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ResponseReceivedExtraInfoEvent {
                pub params: ResponseReceivedExtraInfoEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ResponseReceivedExtraInfoEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "blockedCookies")]
                pub blocked_cookies: Vec<super::BlockedSetCookieWithReason>,
                #[serde(rename = "headers")]
                pub headers: super::Headers,
                #[serde(rename = "resourceIPAddressSpace")]
                pub resource_ip_address_space: super::IPAddressSpace,
                #[serde(default)]
                #[serde(rename = "statusCode")]
                pub status_code: JsUInt,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "headersText")]
                pub headers_text: Option<String>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TrustTokenOperationDoneEvent {
                pub params: TrustTokenOperationDoneEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TrustTokenOperationDoneEventParams {
                #[serde(rename = "status")]
                pub status: super::TrustTokenOperationDoneEventStatusOption,
                #[serde(rename = "type")]
                pub Type: super::TrustTokenOperationType,
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "topLevelOrigin")]
                pub top_level_origin: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "issuerOrigin")]
                pub issuer_origin: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "issuedTokenCount")]
                pub issued_token_count: Option<JsUInt>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SubresourceWebBundleMetadataReceivedEvent {
                pub params: SubresourceWebBundleMetadataReceivedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SubresourceWebBundleMetadataReceivedEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(default)]
                #[serde(rename = "urls")]
                pub urls: Vec<String>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SubresourceWebBundleMetadataErrorEvent {
                pub params: SubresourceWebBundleMetadataErrorEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SubresourceWebBundleMetadataErrorEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(default)]
                #[serde(rename = "errorMessage")]
                pub error_message: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SubresourceWebBundleInnerResponseParsedEvent {
                pub params: SubresourceWebBundleInnerResponseParsedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SubresourceWebBundleInnerResponseParsedEventParams {
                #[serde(rename = "innerRequestId")]
                pub inner_request_id: super::RequestId,
                #[serde(default)]
                #[serde(rename = "innerRequestURL")]
                pub inner_request_url: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "bundleRequestId")]
                pub bundle_request_id: Option<super::RequestId>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SubresourceWebBundleInnerResponseErrorEvent {
                pub params: SubresourceWebBundleInnerResponseErrorEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SubresourceWebBundleInnerResponseErrorEventParams {
                #[serde(rename = "innerRequestId")]
                pub inner_request_id: super::RequestId,
                #[serde(default)]
                #[serde(rename = "innerRequestURL")]
                pub inner_request_url: String,
                #[serde(default)]
                #[serde(rename = "errorMessage")]
                pub error_message: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "bundleRequestId")]
                pub bundle_request_id: Option<super::RequestId>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ReportingApiReportAddedEvent {
                pub params: ReportingApiReportAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ReportingApiReportAddedEventParams {
                #[serde(rename = "report")]
                pub report: super::ReportingApiReport,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ReportingApiReportUpdatedEvent {
                pub params: ReportingApiReportUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ReportingApiReportUpdatedEventParams {
                #[serde(rename = "report")]
                pub report: super::ReportingApiReport,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ReportingApiEndpointsChangedForOriginEvent {
                pub params: ReportingApiEndpointsChangedForOriginEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ReportingApiEndpointsChangedForOriginEventParams {
                #[serde(default)]
                #[serde(rename = "origin")]
                pub origin: String,
                #[serde(rename = "endpoints")]
                pub endpoints: Vec<super::ReportingApiEndpoint>,
            }
        }
    }
    pub mod Overlay {
        use super::types::*;
        use super::Page;
        use super::Runtime;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum LineStylePattern {
            #[serde(rename = "dashed")]
            Dashed,
            #[serde(rename = "dotted")]
            Dotted,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ContrastAlgorithm {
            #[serde(rename = "aa")]
            Aa,
            #[serde(rename = "aaa")]
            Aaa,
            #[serde(rename = "apca")]
            Apca,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ColorFormat {
            #[serde(rename = "rgb")]
            Rgb,
            #[serde(rename = "hsl")]
            Hsl,
            #[serde(rename = "hex")]
            Hex,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum InspectMode {
            #[serde(rename = "searchForNode")]
            SearchForNode,
            #[serde(rename = "searchForUAShadowDOM")]
            SearchForUaShadowDom,
            #[serde(rename = "captureAreaScreenshot")]
            CaptureAreaScreenshot,
            #[serde(rename = "showDistances")]
            ShowDistances,
            #[serde(rename = "none")]
            None,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SourceOrderConfig {
            #[serde(rename = "parentOutlineColor")]
            pub parent_outline_color: DOM::RGBA,
            #[serde(rename = "childOutlineColor")]
            pub child_outline_color: DOM::RGBA,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GridHighlightConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "showGridExtensionLines")]
            pub show_grid_extension_lines: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "showPositiveLineNumbers")]
            pub show_positive_line_numbers: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "showNegativeLineNumbers")]
            pub show_negative_line_numbers: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "showAreaNames")]
            pub show_area_names: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "showLineNames")]
            pub show_line_names: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "showTrackSizes")]
            pub show_track_sizes: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "gridBorderColor")]
            pub grid_border_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "cellBorderColor")]
            pub cell_border_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "rowLineColor")]
            pub row_line_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "columnLineColor")]
            pub column_line_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "gridBorderDash")]
            pub grid_border_dash: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "cellBorderDash")]
            pub cell_border_dash: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "rowLineDash")]
            pub row_line_dash: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "columnLineDash")]
            pub column_line_dash: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "rowGapColor")]
            pub row_gap_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "rowHatchColor")]
            pub row_hatch_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "columnGapColor")]
            pub column_gap_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "columnHatchColor")]
            pub column_hatch_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "areaBorderColor")]
            pub area_border_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "gridBackgroundColor")]
            pub grid_background_color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct FlexContainerHighlightConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "containerBorder")]
            pub container_border: Option<LineStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "lineSeparator")]
            pub line_separator: Option<LineStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "itemSeparator")]
            pub item_separator: Option<LineStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "mainDistributedSpace")]
            pub main_distributed_space: Option<BoxStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "crossDistributedSpace")]
            pub cross_distributed_space: Option<BoxStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "rowGapSpace")]
            pub row_gap_space: Option<BoxStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "columnGapSpace")]
            pub column_gap_space: Option<BoxStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "crossAlignment")]
            pub cross_alignment: Option<LineStyle>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct FlexItemHighlightConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "baseSizeBox")]
            pub base_size_box: Option<BoxStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "baseSizeBorder")]
            pub base_size_border: Option<LineStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "flexibilityArrow")]
            pub flexibility_arrow: Option<LineStyle>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct LineStyle {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "color")]
            pub color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "pattern")]
            pub pattern: Option<LineStylePattern>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct BoxStyle {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "fillColor")]
            pub fill_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "hatchColor")]
            pub hatch_color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct HighlightConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "showInfo")]
            pub show_info: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "showStyles")]
            pub show_styles: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "showRulers")]
            pub show_rulers: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "showAccessibilityInfo")]
            pub show_accessibility_info: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "showExtensionLines")]
            pub show_extension_lines: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "contentColor")]
            pub content_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "paddingColor")]
            pub padding_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "borderColor")]
            pub border_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "marginColor")]
            pub margin_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "eventTargetColor")]
            pub event_target_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "shapeColor")]
            pub shape_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "shapeMarginColor")]
            pub shape_margin_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "cssGridColor")]
            pub css_grid_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "colorFormat")]
            pub color_format: Option<ColorFormat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "gridHighlightConfig")]
            pub grid_highlight_config: Option<GridHighlightConfig>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "flexContainerHighlightConfig")]
            pub flex_container_highlight_config: Option<FlexContainerHighlightConfig>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "flexItemHighlightConfig")]
            pub flex_item_highlight_config: Option<FlexItemHighlightConfig>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "contrastAlgorithm")]
            pub contrast_algorithm: Option<ContrastAlgorithm>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "containerQueryContainerHighlightConfig")]
            pub container_query_container_highlight_config:
                Option<ContainerQueryContainerHighlightConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GridNodeHighlightConfig {
            #[serde(rename = "gridHighlightConfig")]
            pub grid_highlight_config: GridHighlightConfig,
            #[serde(rename = "nodeId")]
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct FlexNodeHighlightConfig {
            #[serde(rename = "flexContainerHighlightConfig")]
            pub flex_container_highlight_config: FlexContainerHighlightConfig,
            #[serde(rename = "nodeId")]
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ScrollSnapContainerHighlightConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "snapportBorder")]
            pub snapport_border: Option<LineStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "snapAreaBorder")]
            pub snap_area_border: Option<LineStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "scrollMarginColor")]
            pub scroll_margin_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "scrollPaddingColor")]
            pub scroll_padding_color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ScrollSnapHighlightConfig {
            #[serde(rename = "scrollSnapContainerHighlightConfig")]
            pub scroll_snap_container_highlight_config: ScrollSnapContainerHighlightConfig,
            #[serde(rename = "nodeId")]
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct HingeConfig {
            #[serde(rename = "rect")]
            pub rect: DOM::Rect,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "contentColor")]
            pub content_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "outlineColor")]
            pub outline_color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ContainerQueryHighlightConfig {
            #[serde(rename = "containerQueryContainerHighlightConfig")]
            pub container_query_container_highlight_config: ContainerQueryContainerHighlightConfig,
            #[serde(rename = "nodeId")]
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ContainerQueryContainerHighlightConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "containerBorder")]
            pub container_border: Option<LineStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "descendantBorder")]
            pub descendant_border: Option<LineStyle>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct IsolatedElementHighlightConfig {
            #[serde(rename = "isolationModeHighlightConfig")]
            pub isolation_mode_highlight_config: IsolationModeHighlightConfig,
            #[serde(rename = "nodeId")]
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct IsolationModeHighlightConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "resizerColor")]
            pub resizer_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "resizerHandleColor")]
            pub resizer_handle_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "maskColor")]
            pub mask_color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetHighlightObjectForTest {
            #[serde(rename = "nodeId")]
            pub node_id: DOM::NodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "includeDistance")]
            pub include_distance: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "includeStyle")]
            pub include_style: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "colorFormat")]
            pub color_format: Option<ColorFormat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "showAccessibilityInfo")]
            pub show_accessibility_info: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetGridHighlightObjectsForTest {
            #[serde(rename = "nodeIds")]
            pub node_ids: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetSourceOrderHighlightObjectForTest {
            #[serde(rename = "nodeId")]
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HideHighlight(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct HighlightFrame {
            #[serde(rename = "frameId")]
            pub frame_id: Page::FrameId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "contentColor")]
            pub content_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "contentOutlineColor")]
            pub content_outline_color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct HighlightNode {
            #[serde(rename = "highlightConfig")]
            pub highlight_config: HighlightConfig,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<DOM::NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "objectId")]
            pub object_id: Option<Runtime::RemoteObjectId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "selector")]
            pub selector: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct HighlightQuad {
            #[serde(rename = "quad")]
            pub quad: DOM::Quad,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "color")]
            pub color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "outlineColor")]
            pub outline_color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct HighlightRect {
            #[serde(default)]
            #[serde(rename = "x")]
            pub x: JsUInt,
            #[serde(default)]
            #[serde(rename = "y")]
            pub y: JsUInt,
            #[serde(default)]
            #[serde(rename = "width")]
            pub width: JsUInt,
            #[serde(default)]
            #[serde(rename = "height")]
            pub height: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "color")]
            pub color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "outlineColor")]
            pub outline_color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct HighlightSourceOrder {
            #[serde(rename = "sourceOrderConfig")]
            pub source_order_config: SourceOrderConfig,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<DOM::NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "backendNodeId")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "objectId")]
            pub object_id: Option<Runtime::RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetInspectMode {
            #[serde(rename = "mode")]
            pub mode: InspectMode,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "highlightConfig")]
            pub highlight_config: Option<HighlightConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetShowAdHighlights {
            #[serde(default)]
            #[serde(rename = "show")]
            pub show: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetPausedInDebuggerMessage {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "message")]
            pub message: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetShowDebugBorders {
            #[serde(default)]
            #[serde(rename = "show")]
            pub show: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetShowFPSCounter {
            #[serde(default)]
            #[serde(rename = "show")]
            pub show: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetShowGridOverlays {
            #[serde(rename = "gridNodeHighlightConfigs")]
            pub grid_node_highlight_configs: Vec<GridNodeHighlightConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetShowFlexOverlays {
            #[serde(rename = "flexNodeHighlightConfigs")]
            pub flex_node_highlight_configs: Vec<FlexNodeHighlightConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetShowScrollSnapOverlays {
            #[serde(rename = "scrollSnapHighlightConfigs")]
            pub scroll_snap_highlight_configs: Vec<ScrollSnapHighlightConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetShowContainerQueryOverlays {
            #[serde(rename = "containerQueryHighlightConfigs")]
            pub container_query_highlight_configs: Vec<ContainerQueryHighlightConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetShowPaintRects {
            #[serde(default)]
            #[serde(rename = "result")]
            pub result: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetShowLayoutShiftRegions {
            #[serde(default)]
            #[serde(rename = "result")]
            pub result: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetShowScrollBottleneckRects {
            #[serde(default)]
            #[serde(rename = "show")]
            pub show: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetShowHitTestBorders {
            #[serde(default)]
            #[serde(rename = "show")]
            pub show: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetShowWebVitals {
            #[serde(default)]
            #[serde(rename = "show")]
            pub show: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetShowViewportSizeOnResize {
            #[serde(default)]
            #[serde(rename = "show")]
            pub show: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetShowHinge {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "hingeConfig")]
            pub hinge_config: Option<HingeConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetShowIsolatedElements {
            #[serde(rename = "isolatedElementHighlightConfigs")]
            pub isolated_element_highlight_configs: Vec<IsolatedElementHighlightConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetHighlightObjectForTestReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetGridHighlightObjectsForTestReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetSourceOrderHighlightObjectForTestReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HideHighlightReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightFrameReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightNodeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightQuadReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightRectReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightSourceOrderReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInspectModeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowAdHighlightsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPausedInDebuggerMessageReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowDebugBordersReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowFPSCounterReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowGridOverlaysReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowFlexOverlaysReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowScrollSnapOverlaysReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowContainerQueryOverlaysReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowPaintRectsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowLayoutShiftRegionsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowScrollBottleneckRectsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowHitTestBordersReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowWebVitalsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowViewportSizeOnResizeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowHingeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowIsolatedElementsReturnObject {}
        impl Method for Disable {
            const NAME: &'static str = "Overlay.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Overlay.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetHighlightObjectForTest {
            const NAME: &'static str = "Overlay.getHighlightObjectForTest";
            type ReturnObject = GetHighlightObjectForTestReturnObject;
        }
        impl Method for GetGridHighlightObjectsForTest {
            const NAME: &'static str = "Overlay.getGridHighlightObjectsForTest";
            type ReturnObject = GetGridHighlightObjectsForTestReturnObject;
        }
        impl Method for GetSourceOrderHighlightObjectForTest {
            const NAME: &'static str = "Overlay.getSourceOrderHighlightObjectForTest";
            type ReturnObject = GetSourceOrderHighlightObjectForTestReturnObject;
        }
        impl Method for HideHighlight {
            const NAME: &'static str = "Overlay.hideHighlight";
            type ReturnObject = HideHighlightReturnObject;
        }
        impl Method for HighlightFrame {
            const NAME: &'static str = "Overlay.highlightFrame";
            type ReturnObject = HighlightFrameReturnObject;
        }
        impl Method for HighlightNode {
            const NAME: &'static str = "Overlay.highlightNode";
            type ReturnObject = HighlightNodeReturnObject;
        }
        impl Method for HighlightQuad {
            const NAME: &'static str = "Overlay.highlightQuad";
            type ReturnObject = HighlightQuadReturnObject;
        }
        impl Method for HighlightRect {
            const NAME: &'static str = "Overlay.highlightRect";
            type ReturnObject = HighlightRectReturnObject;
        }
        impl Method for HighlightSourceOrder {
            const NAME: &'static str = "Overlay.highlightSourceOrder";
            type ReturnObject = HighlightSourceOrderReturnObject;
        }
        impl Method for SetInspectMode {
            const NAME: &'static str = "Overlay.setInspectMode";
            type ReturnObject = SetInspectModeReturnObject;
        }
        impl Method for SetShowAdHighlights {
            const NAME: &'static str = "Overlay.setShowAdHighlights";
            type ReturnObject = SetShowAdHighlightsReturnObject;
        }
        impl Method for SetPausedInDebuggerMessage {
            const NAME: &'static str = "Overlay.setPausedInDebuggerMessage";
            type ReturnObject = SetPausedInDebuggerMessageReturnObject;
        }
        impl Method for SetShowDebugBorders {
            const NAME: &'static str = "Overlay.setShowDebugBorders";
            type ReturnObject = SetShowDebugBordersReturnObject;
        }
        impl Method for SetShowFPSCounter {
            const NAME: &'static str = "Overlay.setShowFPSCounter";
            type ReturnObject = SetShowFPSCounterReturnObject;
        }
        impl Method for SetShowGridOverlays {
            const NAME: &'static str = "Overlay.setShowGridOverlays";
            type ReturnObject = SetShowGridOverlaysReturnObject;
        }
        impl Method for SetShowFlexOverlays {
            const NAME: &'static str = "Overlay.setShowFlexOverlays";
            type ReturnObject = SetShowFlexOverlaysReturnObject;
        }
        impl Method for SetShowScrollSnapOverlays {
            const NAME: &'static str = "Overlay.setShowScrollSnapOverlays";
            type ReturnObject = SetShowScrollSnapOverlaysReturnObject;
        }
        impl Method for SetShowContainerQueryOverlays {
            const NAME: &'static str = "Overlay.setShowContainerQueryOverlays";
            type ReturnObject = SetShowContainerQueryOverlaysReturnObject;
        }
        impl Method for SetShowPaintRects {
            const NAME: &'static str = "Overlay.setShowPaintRects";
            type ReturnObject = SetShowPaintRectsReturnObject;
        }
        impl Method for SetShowLayoutShiftRegions {
            const NAME: &'static str = "Overlay.setShowLayoutShiftRegions";
            type ReturnObject = SetShowLayoutShiftRegionsReturnObject;
        }
        impl Method for SetShowScrollBottleneckRects {
            const NAME: &'static str = "Overlay.setShowScrollBottleneckRects";
            type ReturnObject = SetShowScrollBottleneckRectsReturnObject;
        }
        impl Method for SetShowHitTestBorders {
            const NAME: &'static str = "Overlay.setShowHitTestBorders";
            type ReturnObject = SetShowHitTestBordersReturnObject;
        }
        impl Method for SetShowWebVitals {
            const NAME: &'static str = "Overlay.setShowWebVitals";
            type ReturnObject = SetShowWebVitalsReturnObject;
        }
        impl Method for SetShowViewportSizeOnResize {
            const NAME: &'static str = "Overlay.setShowViewportSizeOnResize";
            type ReturnObject = SetShowViewportSizeOnResizeReturnObject;
        }
        impl Method for SetShowHinge {
            const NAME: &'static str = "Overlay.setShowHinge";
            type ReturnObject = SetShowHingeReturnObject;
        }
        impl Method for SetShowIsolatedElements {
            const NAME: &'static str = "Overlay.setShowIsolatedElements";
            type ReturnObject = SetShowIsolatedElementsReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct InspectNodeRequestedEvent {
                pub params: InspectNodeRequestedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct InspectNodeRequestedEventParams {
                #[serde(rename = "backendNodeId")]
                pub backend_node_id: super::super::DOM::BackendNodeId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodeHighlightRequestedEvent {
                pub params: NodeHighlightRequestedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodeHighlightRequestedEventParams {
                #[serde(rename = "nodeId")]
                pub node_id: super::super::DOM::NodeId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ScreenshotRequestedEvent {
                pub params: ScreenshotRequestedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ScreenshotRequestedEventParams {
                #[serde(rename = "viewport")]
                pub viewport: super::super::Page::Viewport,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct InspectModeCanceledEvent(pub Option<serde_json::Value>);
        }
    }
    pub mod Page {
        use super::types::*;
        use super::Debugger;
        use super::Emulation;
        use super::Network;
        use super::Runtime;
        use super::DOM;
        use super::IO;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type FrameId = String;
        pub type ScriptIdentifier = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum AdFrameType {
            #[serde(rename = "none")]
            None,
            #[serde(rename = "child")]
            Child,
            #[serde(rename = "root")]
            Root,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum AdFrameExplanation {
            #[serde(rename = "ParentIsAd")]
            ParentIsAd,
            #[serde(rename = "CreatedByAdScript")]
            CreatedByAdScript,
            #[serde(rename = "MatchedBlockingRule")]
            MatchedBlockingRule,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SecureContextType {
            #[serde(rename = "Secure")]
            Secure,
            #[serde(rename = "SecureLocalhost")]
            SecureLocalhost,
            #[serde(rename = "InsecureScheme")]
            InsecureScheme,
            #[serde(rename = "InsecureAncestor")]
            InsecureAncestor,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum CrossOriginIsolatedContextType {
            #[serde(rename = "Isolated")]
            Isolated,
            #[serde(rename = "NotIsolated")]
            NotIsolated,
            #[serde(rename = "NotIsolatedFeatureDisabled")]
            NotIsolatedFeatureDisabled,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum GatedAPIFeatures {
            #[serde(rename = "SharedArrayBuffers")]
            SharedArrayBuffers,
            #[serde(rename = "SharedArrayBuffersTransferAllowed")]
            SharedArrayBuffersTransferAllowed,
            #[serde(rename = "PerformanceMeasureMemory")]
            PerformanceMeasureMemory,
            #[serde(rename = "PerformanceProfile")]
            PerformanceProfile,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum PermissionsPolicyFeature {
            #[serde(rename = "accelerometer")]
            Accelerometer,
            #[serde(rename = "ambient-light-sensor")]
            AmbientLightSensor,
            #[serde(rename = "attribution-reporting")]
            AttributionReporting,
            #[serde(rename = "autoplay")]
            Autoplay,
            #[serde(rename = "camera")]
            Camera,
            #[serde(rename = "ch-dpr")]
            ChDpr,
            #[serde(rename = "ch-device-memory")]
            ChDeviceMemory,
            #[serde(rename = "ch-downlink")]
            ChDownlink,
            #[serde(rename = "ch-ect")]
            ChEct,
            #[serde(rename = "ch-prefers-color-scheme")]
            ChPrefersColorScheme,
            #[serde(rename = "ch-rtt")]
            ChRtt,
            #[serde(rename = "ch-ua")]
            ChUa,
            #[serde(rename = "ch-ua-arch")]
            ChUaArch,
            #[serde(rename = "ch-ua-bitness")]
            ChUaBitness,
            #[serde(rename = "ch-ua-platform")]
            ChUaPlatform,
            #[serde(rename = "ch-ua-model")]
            ChUaModel,
            #[serde(rename = "ch-ua-mobile")]
            ChUaMobile,
            #[serde(rename = "ch-ua-full-version")]
            ChUaFullVersion,
            #[serde(rename = "ch-ua-full-version-list")]
            ChUaFullVersionList,
            #[serde(rename = "ch-ua-platform-version")]
            ChUaPlatformVersion,
            #[serde(rename = "ch-ua-reduced")]
            ChUaReduced,
            #[serde(rename = "ch-viewport-height")]
            ChViewportHeight,
            #[serde(rename = "ch-viewport-width")]
            ChViewportWidth,
            #[serde(rename = "ch-width")]
            ChWidth,
            #[serde(rename = "clipboard-read")]
            ClipboardRead,
            #[serde(rename = "clipboard-write")]
            ClipboardWrite,
            #[serde(rename = "cross-origin-isolated")]
            CrossOriginIsolated,
            #[serde(rename = "direct-sockets")]
            DirectSockets,
            #[serde(rename = "display-capture")]
            DisplayCapture,
            #[serde(rename = "document-domain")]
            DocumentDomain,
            #[serde(rename = "encrypted-media")]
            EncryptedMedia,
            #[serde(rename = "execution-while-out-of-viewport")]
            ExecutionWhileOutOfViewport,
            #[serde(rename = "execution-while-not-rendered")]
            ExecutionWhileNotRendered,
            #[serde(rename = "focus-without-user-activation")]
            FocusWithoutUserActivation,
            #[serde(rename = "fullscreen")]
            Fullscreen,
            #[serde(rename = "frobulate")]
            Frobulate,
            #[serde(rename = "gamepad")]
            Gamepad,
            #[serde(rename = "geolocation")]
            Geolocation,
            #[serde(rename = "gyroscope")]
            Gyroscope,
            #[serde(rename = "hid")]
            Hid,
            #[serde(rename = "idle-detection")]
            IdleDetection,
            #[serde(rename = "interest-cohort")]
            InterestCohort,
            #[serde(rename = "join-ad-interest-group")]
            JoinAdInterestGroup,
            #[serde(rename = "keyboard-map")]
            KeyboardMap,
            #[serde(rename = "magnetometer")]
            Magnetometer,
            #[serde(rename = "microphone")]
            Microphone,
            #[serde(rename = "midi")]
            Midi,
            #[serde(rename = "otp-credentials")]
            OtpCredentials,
            #[serde(rename = "payment")]
            Payment,
            #[serde(rename = "picture-in-picture")]
            PictureInPicture,
            #[serde(rename = "publickey-credentials-get")]
            PublickeyCredentialsGet,
            #[serde(rename = "run-ad-auction")]
            RunAdAuction,
            #[serde(rename = "screen-wake-lock")]
            ScreenWakeLock,
            #[serde(rename = "serial")]
            Serial,
            #[serde(rename = "shared-autofill")]
            SharedAutofill,
            #[serde(rename = "storage-access-api")]
            StorageAccessApi,
            #[serde(rename = "sync-xhr")]
            SyncXhr,
            #[serde(rename = "trust-token-redemption")]
            TrustTokenRedemption,
            #[serde(rename = "usb")]
            Usb,
            #[serde(rename = "vertical-scroll")]
            VerticalScroll,
            #[serde(rename = "web-share")]
            WebShare,
            #[serde(rename = "window-placement")]
            WindowPlacement,
            #[serde(rename = "xr-spatial-tracking")]
            XrSpatialTracking,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum PermissionsPolicyBlockReason {
            #[serde(rename = "Header")]
            Header,
            #[serde(rename = "IframeAttribute")]
            IframeAttribute,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum OriginTrialTokenStatus {
            #[serde(rename = "Success")]
            Success,
            #[serde(rename = "NotSupported")]
            NotSupported,
            #[serde(rename = "Insecure")]
            Insecure,
            #[serde(rename = "Expired")]
            Expired,
            #[serde(rename = "WrongOrigin")]
            WrongOrigin,
            #[serde(rename = "InvalidSignature")]
            InvalidSignature,
            #[serde(rename = "Malformed")]
            Malformed,
            #[serde(rename = "WrongVersion")]
            WrongVersion,
            #[serde(rename = "FeatureDisabled")]
            FeatureDisabled,
            #[serde(rename = "TokenDisabled")]
            TokenDisabled,
            #[serde(rename = "FeatureDisabledForUser")]
            FeatureDisabledForUser,
            #[serde(rename = "UnknownTrial")]
            UnknownTrial,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum OriginTrialStatus {
            #[serde(rename = "Enabled")]
            Enabled,
            #[serde(rename = "ValidTokenNotProvided")]
            ValidTokenNotProvided,
            #[serde(rename = "OSNotSupported")]
            OsNotSupported,
            #[serde(rename = "TrialNotAllowed")]
            TrialNotAllowed,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum OriginTrialUsageRestriction {
            #[serde(rename = "None")]
            None,
            #[serde(rename = "Subset")]
            Subset,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum TransitionType {
            #[serde(rename = "link")]
            Link,
            #[serde(rename = "typed")]
            Typed,
            #[serde(rename = "address_bar")]
            AddressBar,
            #[serde(rename = "auto_bookmark")]
            AutoBookmark,
            #[serde(rename = "auto_subframe")]
            AutoSubframe,
            #[serde(rename = "manual_subframe")]
            ManualSubframe,
            #[serde(rename = "generated")]
            Generated,
            #[serde(rename = "auto_toplevel")]
            AutoToplevel,
            #[serde(rename = "form_submit")]
            FormSubmit,
            #[serde(rename = "reload")]
            Reload,
            #[serde(rename = "keyword")]
            Keyword,
            #[serde(rename = "keyword_generated")]
            KeywordGenerated,
            #[serde(rename = "other")]
            Other,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum DialogType {
            #[serde(rename = "alert")]
            Alert,
            #[serde(rename = "confirm")]
            Confirm,
            #[serde(rename = "prompt")]
            Prompt,
            #[serde(rename = "beforeunload")]
            Beforeunload,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ClientNavigationReason {
            #[serde(rename = "formSubmissionGet")]
            FormSubmissionGet,
            #[serde(rename = "formSubmissionPost")]
            FormSubmissionPost,
            #[serde(rename = "httpHeaderRefresh")]
            HttpHeaderRefresh,
            #[serde(rename = "scriptInitiated")]
            ScriptInitiated,
            #[serde(rename = "metaTagRefresh")]
            MetaTagRefresh,
            #[serde(rename = "pageBlockInterstitial")]
            PageBlockInterstitial,
            #[serde(rename = "reload")]
            Reload,
            #[serde(rename = "anchorClick")]
            AnchorClick,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ClientNavigationDisposition {
            #[serde(rename = "currentTab")]
            CurrentTab,
            #[serde(rename = "newTab")]
            NewTab,
            #[serde(rename = "newWindow")]
            NewWindow,
            #[serde(rename = "download")]
            Download,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ReferrerPolicy {
            #[serde(rename = "noReferrer")]
            NoReferrer,
            #[serde(rename = "noReferrerWhenDowngrade")]
            NoReferrerWhenDowngrade,
            #[serde(rename = "origin")]
            Origin,
            #[serde(rename = "originWhenCrossOrigin")]
            OriginWhenCrossOrigin,
            #[serde(rename = "sameOrigin")]
            SameOrigin,
            #[serde(rename = "strictOrigin")]
            StrictOrigin,
            #[serde(rename = "strictOriginWhenCrossOrigin")]
            StrictOriginWhenCrossOrigin,
            #[serde(rename = "unsafeUrl")]
            UnsafeUrl,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum NavigationType {
            #[serde(rename = "Navigation")]
            Navigation,
            #[serde(rename = "BackForwardCacheRestore")]
            BackForwardCacheRestore,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum BackForwardCacheNotRestoredReason {
            #[serde(rename = "NotMainFrame")]
            NotMainFrame,
            #[serde(rename = "BackForwardCacheDisabled")]
            BackForwardCacheDisabled,
            #[serde(rename = "RelatedActiveContentsExist")]
            RelatedActiveContentsExist,
            #[serde(rename = "HTTPStatusNotOK")]
            HttpStatusNotOk,
            #[serde(rename = "SchemeNotHTTPOrHTTPS")]
            SchemeNotHttpOrHttps,
            #[serde(rename = "Loading")]
            Loading,
            #[serde(rename = "WasGrantedMediaAccess")]
            WasGrantedMediaAccess,
            #[serde(rename = "DisableForRenderFrameHostCalled")]
            DisableForRenderFrameHostCalled,
            #[serde(rename = "DomainNotAllowed")]
            DomainNotAllowed,
            #[serde(rename = "HTTPMethodNotGET")]
            HttpMethodNotGet,
            #[serde(rename = "SubframeIsNavigating")]
            SubframeIsNavigating,
            #[serde(rename = "Timeout")]
            Timeout,
            #[serde(rename = "CacheLimit")]
            CacheLimit,
            #[serde(rename = "JavaScriptExecution")]
            JavaScriptExecution,
            #[serde(rename = "RendererProcessKilled")]
            RendererProcessKilled,
            #[serde(rename = "RendererProcessCrashed")]
            RendererProcessCrashed,
            #[serde(rename = "GrantedMediaStreamAccess")]
            GrantedMediaStreamAccess,
            #[serde(rename = "SchedulerTrackedFeatureUsed")]
            SchedulerTrackedFeatureUsed,
            #[serde(rename = "ConflictingBrowsingInstance")]
            ConflictingBrowsingInstance,
            #[serde(rename = "CacheFlushed")]
            CacheFlushed,
            #[serde(rename = "ServiceWorkerVersionActivation")]
            ServiceWorkerVersionActivation,
            #[serde(rename = "SessionRestored")]
            SessionRestored,
            #[serde(rename = "ServiceWorkerPostMessage")]
            ServiceWorkerPostMessage,
            #[serde(rename = "EnteredBackForwardCacheBeforeServiceWorkerHostAdded")]
            EnteredBackForwardCacheBeforeServiceWorkerHostAdded,
            #[serde(rename = "RenderFrameHostReused_SameSite")]
            RenderFrameHostReusedSameSite,
            #[serde(rename = "RenderFrameHostReused_CrossSite")]
            RenderFrameHostReusedCrossSite,
            #[serde(rename = "ServiceWorkerClaim")]
            ServiceWorkerClaim,
            #[serde(rename = "IgnoreEventAndEvict")]
            IgnoreEventAndEvict,
            #[serde(rename = "HaveInnerContents")]
            HaveInnerContents,
            #[serde(rename = "TimeoutPuttingInCache")]
            TimeoutPuttingInCache,
            #[serde(rename = "BackForwardCacheDisabledByLowMemory")]
            BackForwardCacheDisabledByLowMemory,
            #[serde(rename = "BackForwardCacheDisabledByCommandLine")]
            BackForwardCacheDisabledByCommandLine,
            #[serde(rename = "NetworkRequestDatapipeDrainedAsBytesConsumer")]
            NetworkRequestDatapipeDrainedAsBytesConsumer,
            #[serde(rename = "NetworkRequestRedirected")]
            NetworkRequestRedirected,
            #[serde(rename = "NetworkRequestTimeout")]
            NetworkRequestTimeout,
            #[serde(rename = "NetworkExceedsBufferLimit")]
            NetworkExceedsBufferLimit,
            #[serde(rename = "NavigationCancelledWhileRestoring")]
            NavigationCancelledWhileRestoring,
            #[serde(rename = "NotMostRecentNavigationEntry")]
            NotMostRecentNavigationEntry,
            #[serde(rename = "BackForwardCacheDisabledForPrerender")]
            BackForwardCacheDisabledForPrerender,
            #[serde(rename = "UserAgentOverrideDiffers")]
            UserAgentOverrideDiffers,
            #[serde(rename = "ForegroundCacheLimit")]
            ForegroundCacheLimit,
            #[serde(rename = "BrowsingInstanceNotSwapped")]
            BrowsingInstanceNotSwapped,
            #[serde(rename = "BackForwardCacheDisabledForDelegate")]
            BackForwardCacheDisabledForDelegate,
            #[serde(rename = "OptInUnloadHeaderNotPresent")]
            OptInUnloadHeaderNotPresent,
            #[serde(rename = "UnloadHandlerExistsInMainFrame")]
            UnloadHandlerExistsInMainFrame,
            #[serde(rename = "UnloadHandlerExistsInSubFrame")]
            UnloadHandlerExistsInSubFrame,
            #[serde(rename = "ServiceWorkerUnregistration")]
            ServiceWorkerUnregistration,
            #[serde(rename = "CacheControlNoStore")]
            CacheControlNoStore,
            #[serde(rename = "CacheControlNoStoreCookieModified")]
            CacheControlNoStoreCookieModified,
            #[serde(rename = "CacheControlNoStoreHTTPOnlyCookieModified")]
            CacheControlNoStoreHttpOnlyCookieModified,
            #[serde(rename = "NoResponseHead")]
            NoResponseHead,
            #[serde(rename = "Unknown")]
            Unknown,
            #[serde(rename = "ActivationNavigationsDisallowedForBug1234857")]
            ActivationNavigationsDisallowedForBug1234857,
            #[serde(rename = "WebSocket")]
            WebSocket,
            #[serde(rename = "WebTransport")]
            WebTransport,
            #[serde(rename = "WebRTC")]
            WebRtc,
            #[serde(rename = "MainResourceHasCacheControlNoStore")]
            MainResourceHasCacheControlNoStore,
            #[serde(rename = "MainResourceHasCacheControlNoCache")]
            MainResourceHasCacheControlNoCache,
            #[serde(rename = "SubresourceHasCacheControlNoStore")]
            SubresourceHasCacheControlNoStore,
            #[serde(rename = "SubresourceHasCacheControlNoCache")]
            SubresourceHasCacheControlNoCache,
            #[serde(rename = "ContainsPlugins")]
            ContainsPlugins,
            #[serde(rename = "DocumentLoaded")]
            DocumentLoaded,
            #[serde(rename = "DedicatedWorkerOrWorklet")]
            DedicatedWorkerOrWorklet,
            #[serde(rename = "OutstandingNetworkRequestOthers")]
            OutstandingNetworkRequestOthers,
            #[serde(rename = "OutstandingIndexedDBTransaction")]
            OutstandingIndexedDbTransaction,
            #[serde(rename = "RequestedNotificationsPermission")]
            RequestedNotificationsPermission,
            #[serde(rename = "RequestedMIDIPermission")]
            RequestedMidiPermission,
            #[serde(rename = "RequestedAudioCapturePermission")]
            RequestedAudioCapturePermission,
            #[serde(rename = "RequestedVideoCapturePermission")]
            RequestedVideoCapturePermission,
            #[serde(rename = "RequestedBackForwardCacheBlockedSensors")]
            RequestedBackForwardCacheBlockedSensors,
            #[serde(rename = "RequestedBackgroundWorkPermission")]
            RequestedBackgroundWorkPermission,
            #[serde(rename = "BroadcastChannel")]
            BroadcastChannel,
            #[serde(rename = "IndexedDBConnection")]
            IndexedDbConnection,
            #[serde(rename = "WebXR")]
            WebXr,
            #[serde(rename = "SharedWorker")]
            SharedWorker,
            #[serde(rename = "WebLocks")]
            WebLocks,
            #[serde(rename = "WebHID")]
            WebHid,
            #[serde(rename = "WebShare")]
            WebShare,
            #[serde(rename = "RequestedStorageAccessGrant")]
            RequestedStorageAccessGrant,
            #[serde(rename = "WebNfc")]
            WebNfc,
            #[serde(rename = "OutstandingNetworkRequestFetch")]
            OutstandingNetworkRequestFetch,
            #[serde(rename = "OutstandingNetworkRequestXHR")]
            OutstandingNetworkRequestXhr,
            #[serde(rename = "AppBanner")]
            AppBanner,
            #[serde(rename = "Printing")]
            Printing,
            #[serde(rename = "WebDatabase")]
            WebDatabase,
            #[serde(rename = "PictureInPicture")]
            PictureInPicture,
            #[serde(rename = "Portal")]
            Portal,
            #[serde(rename = "SpeechRecognizer")]
            SpeechRecognizer,
            #[serde(rename = "IdleManager")]
            IdleManager,
            #[serde(rename = "PaymentManager")]
            PaymentManager,
            #[serde(rename = "SpeechSynthesis")]
            SpeechSynthesis,
            #[serde(rename = "KeyboardLock")]
            KeyboardLock,
            #[serde(rename = "WebOTPService")]
            WebOtpService,
            #[serde(rename = "OutstandingNetworkRequestDirectSocket")]
            OutstandingNetworkRequestDirectSocket,
            #[serde(rename = "InjectedJavascript")]
            InjectedJavascript,
            #[serde(rename = "InjectedStyleSheet")]
            InjectedStyleSheet,
            #[serde(rename = "Dummy")]
            Dummy,
            #[serde(rename = "ContentSecurityHandler")]
            ContentSecurityHandler,
            #[serde(rename = "ContentWebAuthenticationAPI")]
            ContentWebAuthenticationApi,
            #[serde(rename = "ContentFileChooser")]
            ContentFileChooser,
            #[serde(rename = "ContentSerial")]
            ContentSerial,
            #[serde(rename = "ContentFileSystemAccess")]
            ContentFileSystemAccess,
            #[serde(rename = "ContentMediaDevicesDispatcherHost")]
            ContentMediaDevicesDispatcherHost,
            #[serde(rename = "ContentWebBluetooth")]
            ContentWebBluetooth,
            #[serde(rename = "ContentWebUSB")]
            ContentWebUsb,
            #[serde(rename = "ContentMediaSession")]
            ContentMediaSession,
            #[serde(rename = "ContentMediaSessionService")]
            ContentMediaSessionService,
            #[serde(rename = "EmbedderPopupBlockerTabHelper")]
            EmbedderPopupBlockerTabHelper,
            #[serde(rename = "EmbedderSafeBrowsingTriggeredPopupBlocker")]
            EmbedderSafeBrowsingTriggeredPopupBlocker,
            #[serde(rename = "EmbedderSafeBrowsingThreatDetails")]
            EmbedderSafeBrowsingThreatDetails,
            #[serde(rename = "EmbedderAppBannerManager")]
            EmbedderAppBannerManager,
            #[serde(rename = "EmbedderDomDistillerViewerSource")]
            EmbedderDomDistillerViewerSource,
            #[serde(rename = "EmbedderDomDistillerSelfDeletingRequestDelegate")]
            EmbedderDomDistillerSelfDeletingRequestDelegate,
            #[serde(rename = "EmbedderOomInterventionTabHelper")]
            EmbedderOomInterventionTabHelper,
            #[serde(rename = "EmbedderOfflinePage")]
            EmbedderOfflinePage,
            #[serde(rename = "EmbedderChromePasswordManagerClientBindCredentialManager")]
            EmbedderChromePasswordManagerClientBindCredentialManager,
            #[serde(rename = "EmbedderPermissionRequestManager")]
            EmbedderPermissionRequestManager,
            #[serde(rename = "EmbedderModalDialog")]
            EmbedderModalDialog,
            #[serde(rename = "EmbedderExtensions")]
            EmbedderExtensions,
            #[serde(rename = "EmbedderExtensionMessaging")]
            EmbedderExtensionMessaging,
            #[serde(rename = "EmbedderExtensionMessagingForOpenPort")]
            EmbedderExtensionMessagingForOpenPort,
            #[serde(rename = "EmbedderExtensionSentMessageToCachedFrame")]
            EmbedderExtensionSentMessageToCachedFrame,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum BackForwardCacheNotRestoredReasonType {
            #[serde(rename = "SupportPending")]
            SupportPending,
            #[serde(rename = "PageSupportNeeded")]
            PageSupportNeeded,
            #[serde(rename = "Circumstantial")]
            Circumstantial,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum CaptureScreenshotFormatOption {
            #[serde(rename = "jpeg")]
            Jpeg,
            #[serde(rename = "png")]
            Png,
            #[serde(rename = "webp")]
            Webp,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum CaptureSnapshotFormatOption {
            #[serde(rename = "mhtml")]
            Mhtml,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum PrintToPDFTransfer_modeOption {
            #[serde(rename = "ReturnAsBase64")]
            ReturnAsBase64,
            #[serde(rename = "ReturnAsStream")]
            ReturnAsStream,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SetDownloadBehaviorBehaviorOption {
            #[serde(rename = "deny")]
            Deny,
            #[serde(rename = "allow")]
            Allow,
            #[serde(rename = "default")]
            Default,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SetTouchEmulationEnabledConfigurationOption {
            #[serde(rename = "mobile")]
            Mobile,
            #[serde(rename = "desktop")]
            Desktop,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum StartScreencastFormatOption {
            #[serde(rename = "jpeg")]
            Jpeg,
            #[serde(rename = "png")]
            Png,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SetWebLifecycleStateStateOption {
            #[serde(rename = "frozen")]
            Frozen,
            #[serde(rename = "active")]
            Active,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SetSPCTransactionModeModeOption {
            #[serde(rename = "none")]
            None,
            #[serde(rename = "autoaccept")]
            Autoaccept,
            #[serde(rename = "autoreject")]
            Autoreject,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum FileChooserOpenedEventModeOption {
            #[serde(rename = "selectSingle")]
            SelectSingle,
            #[serde(rename = "selectMultiple")]
            SelectMultiple,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum FrameDetachedEventReasonOption {
            #[serde(rename = "remove")]
            Remove,
            #[serde(rename = "swap")]
            Swap,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum DownloadProgressEventStateOption {
            #[serde(rename = "inProgress")]
            InProgress,
            #[serde(rename = "completed")]
            Completed,
            #[serde(rename = "canceled")]
            Canceled,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AdFrameStatus {
            #[serde(rename = "adFrameType")]
            pub ad_frame_Type: AdFrameType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "explanations")]
            pub explanations: Option<Vec<AdFrameExplanation>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PermissionsPolicyBlockLocator {
            #[serde(rename = "frameId")]
            pub frame_id: FrameId,
            #[serde(rename = "blockReason")]
            pub block_reason: PermissionsPolicyBlockReason,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PermissionsPolicyFeatureState {
            #[serde(rename = "feature")]
            pub feature: PermissionsPolicyFeature,
            #[serde(default)]
            #[serde(rename = "allowed")]
            pub allowed: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "locator")]
            pub locator: Option<PermissionsPolicyBlockLocator>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct OriginTrialToken {
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: String,
            #[serde(default)]
            #[serde(rename = "matchSubDomains")]
            pub match_sub_domains: bool,
            #[serde(default)]
            #[serde(rename = "trialName")]
            pub trial_name: String,
            #[serde(rename = "expiryTime")]
            pub expiry_time: Network::TimeSinceEpoch,
            #[serde(default)]
            #[serde(rename = "isThirdParty")]
            pub is_third_party: bool,
            #[serde(rename = "usageRestriction")]
            pub usage_restriction: OriginTrialUsageRestriction,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct OriginTrialTokenWithStatus {
            #[serde(default)]
            #[serde(rename = "rawTokenText")]
            pub raw_token_text: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "parsedToken")]
            pub parsed_token: Option<OriginTrialToken>,
            #[serde(rename = "status")]
            pub status: OriginTrialTokenStatus,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct OriginTrial {
            #[serde(default)]
            #[serde(rename = "trialName")]
            pub trial_name: String,
            #[serde(rename = "status")]
            pub status: OriginTrialStatus,
            #[serde(rename = "tokensWithStatus")]
            pub tokens_with_status: Vec<OriginTrialTokenWithStatus>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Frame {
            #[serde(rename = "id")]
            pub id: FrameId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "parentId")]
            pub parent_id: Option<FrameId>,
            #[serde(rename = "loaderId")]
            pub loader_id: Network::LoaderId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: Option<String>,
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "urlFragment")]
            pub url_fragment: Option<String>,
            #[serde(default)]
            #[serde(rename = "domainAndRegistry")]
            pub domain_and_registry: String,
            #[serde(default)]
            #[serde(rename = "securityOrigin")]
            pub security_origin: String,
            #[serde(default)]
            #[serde(rename = "mimeType")]
            pub mime_type: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "unreachableUrl")]
            pub unreachable_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "adFrameStatus")]
            pub ad_frame_status: Option<AdFrameStatus>,
            #[serde(rename = "secureContextType")]
            pub secure_context_Type: SecureContextType,
            #[serde(rename = "crossOriginIsolatedContextType")]
            pub cross_origin_isolated_context_Type: CrossOriginIsolatedContextType,
            #[serde(rename = "gatedAPIFeatures")]
            pub gated_api_features: Vec<GatedAPIFeatures>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct FrameResource {
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(rename = "type")]
            pub Type: Network::ResourceType,
            #[serde(default)]
            #[serde(rename = "mimeType")]
            pub mime_type: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "lastModified")]
            pub last_modified: Option<Network::TimeSinceEpoch>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "contentSize")]
            pub content_size: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "failed")]
            pub failed: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "canceled")]
            pub canceled: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct FrameResourceTree {
            #[serde(rename = "frame")]
            pub frame: Frame,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "childFrames")]
            pub child_frames: Option<Vec<FrameResourceTree>>,
            #[serde(rename = "resources")]
            pub resources: Vec<FrameResource>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct FrameTree {
            #[serde(rename = "frame")]
            pub frame: Frame,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "childFrames")]
            pub child_frames: Option<Vec<FrameTree>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct NavigationEntry {
            #[serde(default)]
            #[serde(rename = "id")]
            pub id: JsUInt,
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(default)]
            #[serde(rename = "userTypedURL")]
            pub user_typed_url: String,
            #[serde(default)]
            #[serde(rename = "title")]
            pub title: String,
            #[serde(rename = "transitionType")]
            pub transition_Type: TransitionType,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ScreencastFrameMetadata {
            #[serde(default)]
            #[serde(rename = "offsetTop")]
            pub offset_top: JsFloat,
            #[serde(default)]
            #[serde(rename = "pageScaleFactor")]
            pub page_scale_factor: JsFloat,
            #[serde(default)]
            #[serde(rename = "deviceWidth")]
            pub device_width: JsFloat,
            #[serde(default)]
            #[serde(rename = "deviceHeight")]
            pub device_height: JsFloat,
            #[serde(default)]
            #[serde(rename = "scrollOffsetX")]
            pub scroll_offset_x: JsFloat,
            #[serde(default)]
            #[serde(rename = "scrollOffsetY")]
            pub scroll_offset_y: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "timestamp")]
            pub timestamp: Option<Network::TimeSinceEpoch>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AppManifestError {
            #[serde(default)]
            #[serde(rename = "message")]
            pub message: String,
            #[serde(default)]
            #[serde(rename = "critical")]
            pub critical: JsUInt,
            #[serde(default)]
            #[serde(rename = "line")]
            pub line: JsUInt,
            #[serde(default)]
            #[serde(rename = "column")]
            pub column: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AppManifestParsedProperties {
            #[serde(default)]
            #[serde(rename = "scope")]
            pub scope: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct LayoutViewport {
            #[serde(default)]
            #[serde(rename = "pageX")]
            pub page_x: JsUInt,
            #[serde(default)]
            #[serde(rename = "pageY")]
            pub page_y: JsUInt,
            #[serde(default)]
            #[serde(rename = "clientWidth")]
            pub client_width: JsUInt,
            #[serde(default)]
            #[serde(rename = "clientHeight")]
            pub client_height: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct VisualViewport {
            #[serde(default)]
            #[serde(rename = "offsetX")]
            pub offset_x: JsFloat,
            #[serde(default)]
            #[serde(rename = "offsetY")]
            pub offset_y: JsFloat,
            #[serde(default)]
            #[serde(rename = "pageX")]
            pub page_x: JsFloat,
            #[serde(default)]
            #[serde(rename = "pageY")]
            pub page_y: JsFloat,
            #[serde(default)]
            #[serde(rename = "clientWidth")]
            pub client_width: JsFloat,
            #[serde(default)]
            #[serde(rename = "clientHeight")]
            pub client_height: JsFloat,
            #[serde(default)]
            #[serde(rename = "scale")]
            pub scale: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "zoom")]
            pub zoom: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Viewport {
            #[serde(default)]
            #[serde(rename = "x")]
            pub x: JsFloat,
            #[serde(default)]
            #[serde(rename = "y")]
            pub y: JsFloat,
            #[serde(default)]
            #[serde(rename = "width")]
            pub width: JsFloat,
            #[serde(default)]
            #[serde(rename = "height")]
            pub height: JsFloat,
            #[serde(default)]
            #[serde(rename = "scale")]
            pub scale: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct FontFamilies {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "standard")]
            pub standard: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "fixed")]
            pub fixed: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "serif")]
            pub serif: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "sansSerif")]
            pub sans_serif: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "cursive")]
            pub cursive: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "fantasy")]
            pub fantasy: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "pictograph")]
            pub pictograph: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct FontSizes {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "standard")]
            pub standard: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "fixed")]
            pub fixed: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct InstallabilityErrorArgument {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct InstallabilityError {
            #[serde(default)]
            #[serde(rename = "errorId")]
            pub error_id: String,
            #[serde(rename = "errorArguments")]
            pub error_arguments: Vec<InstallabilityErrorArgument>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CompilationCacheParams {
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "eager")]
            pub eager: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct BackForwardCacheNotRestoredExplanation {
            #[serde(rename = "type")]
            pub Type: BackForwardCacheNotRestoredReasonType,
            #[serde(rename = "reason")]
            pub reason: BackForwardCacheNotRestoredReason,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AddScriptToEvaluateOnLoad {
            #[serde(default)]
            #[serde(rename = "scriptSource")]
            pub script_source: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AddScriptToEvaluateOnNewDocument {
            #[serde(default)]
            #[serde(rename = "source")]
            pub source: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "worldName")]
            pub world_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "includeCommandLineAPI")]
            pub include_command_line_api: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BringToFront(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CaptureScreenshot {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "format")]
            pub format: Option<CaptureScreenshotFormatOption>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "quality")]
            pub quality: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "clip")]
            pub clip: Option<Viewport>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "fromSurface")]
            pub from_surface: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "captureBeyondViewport")]
            pub capture_beyond_viewport: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CaptureSnapshot {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "format")]
            pub format: Option<CaptureSnapshotFormatOption>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDeviceMetricsOverride(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDeviceOrientationOverride(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearGeolocationOverride(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CreateIsolatedWorld {
            #[serde(rename = "frameId")]
            pub frame_id: FrameId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "worldName")]
            pub world_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "grantUniveralAccess")]
            pub grant_univeral_access: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DeleteCookie {
            #[serde(default)]
            #[serde(rename = "cookieName")]
            pub cookie_name: String,
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetAppManifest(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetInstallabilityErrors(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetManifestIcons(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetAppId(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCookies(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetFrameTree(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetLayoutMetrics(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetNavigationHistory(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResetNavigationHistory(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetResourceContent {
            #[serde(rename = "frameId")]
            pub frame_id: FrameId,
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetResourceTree(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct HandleJavaScriptDialog {
            #[serde(default)]
            #[serde(rename = "accept")]
            pub accept: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "promptText")]
            pub prompt_text: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Navigate {
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "referrer")]
            pub referrer: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "transitionType")]
            pub transition_Type: Option<TransitionType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "frameId")]
            pub frame_id: Option<FrameId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "referrerPolicy")]
            pub referrer_policy: Option<ReferrerPolicy>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct NavigateToHistoryEntry {
            #[serde(default)]
            #[serde(rename = "entryId")]
            pub entry_id: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PrintToPDF {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "landscape")]
            pub landscape: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "displayHeaderFooter")]
            pub display_header_footer: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "printBackground")]
            pub print_background: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "scale")]
            pub scale: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "paperWidth")]
            pub paper_width: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "paperHeight")]
            pub paper_height: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "marginTop")]
            pub margin_top: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "marginBottom")]
            pub margin_bottom: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "marginLeft")]
            pub margin_left: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "marginRight")]
            pub margin_right: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "pageRanges")]
            pub page_ranges: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "ignoreInvalidPageRanges")]
            pub ignore_invalid_page_ranges: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "headerTemplate")]
            pub header_template: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "footerTemplate")]
            pub footer_template: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "preferCSSPageSize")]
            pub prefer_css_page_size: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "transferMode")]
            pub transfer_mode: Option<PrintToPDFTransfer_modeOption>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Reload {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "ignoreCache")]
            pub ignore_cache: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "scriptToEvaluateOnLoad")]
            pub script_to_evaluate_on_load: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RemoveScriptToEvaluateOnLoad {
            #[serde(rename = "identifier")]
            pub identifier: ScriptIdentifier,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RemoveScriptToEvaluateOnNewDocument {
            #[serde(rename = "identifier")]
            pub identifier: ScriptIdentifier,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ScreencastFrameAck {
            #[serde(default)]
            #[serde(rename = "sessionId")]
            pub session_id: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SearchInResource {
            #[serde(rename = "frameId")]
            pub frame_id: FrameId,
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(default)]
            #[serde(rename = "query")]
            pub query: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "caseSensitive")]
            pub case_sensitive: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "isRegex")]
            pub is_regex: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetAdBlockingEnabled {
            #[serde(default)]
            #[serde(rename = "enabled")]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetBypassCSP {
            #[serde(default)]
            #[serde(rename = "enabled")]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetPermissionsPolicyState {
            #[serde(rename = "frameId")]
            pub frame_id: FrameId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetOriginTrials {
            #[serde(rename = "frameId")]
            pub frame_id: FrameId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetDeviceMetricsOverride {
            #[serde(default)]
            #[serde(rename = "width")]
            pub width: JsUInt,
            #[serde(default)]
            #[serde(rename = "height")]
            pub height: JsUInt,
            #[serde(default)]
            #[serde(rename = "deviceScaleFactor")]
            pub device_scale_factor: JsFloat,
            #[serde(default)]
            #[serde(rename = "mobile")]
            pub mobile: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "scale")]
            pub scale: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "screenWidth")]
            pub screen_width: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "screenHeight")]
            pub screen_height: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "positionX")]
            pub position_x: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "positionY")]
            pub position_y: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "dontSetVisibleSize")]
            pub dont_set_visible_size: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "screenOrientation")]
            pub screen_orientation: Option<Emulation::ScreenOrientation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "viewport")]
            pub viewport: Option<Viewport>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetDeviceOrientationOverride {
            #[serde(default)]
            #[serde(rename = "alpha")]
            pub alpha: JsFloat,
            #[serde(default)]
            #[serde(rename = "beta")]
            pub beta: JsFloat,
            #[serde(default)]
            #[serde(rename = "gamma")]
            pub gamma: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetFontFamilies {
            #[serde(rename = "fontFamilies")]
            pub font_families: FontFamilies,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetFontSizes {
            #[serde(rename = "fontSizes")]
            pub font_sizes: FontSizes,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetDocumentContent {
            #[serde(rename = "frameId")]
            pub frame_id: FrameId,
            #[serde(default)]
            #[serde(rename = "html")]
            pub html: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetDownloadBehavior {
            #[serde(rename = "behavior")]
            pub behavior: SetDownloadBehaviorBehaviorOption,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "downloadPath")]
            pub download_path: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetGeolocationOverride {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "latitude")]
            pub latitude: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "longitude")]
            pub longitude: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "accuracy")]
            pub accuracy: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetLifecycleEventsEnabled {
            #[serde(default)]
            #[serde(rename = "enabled")]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetTouchEmulationEnabled {
            #[serde(default)]
            #[serde(rename = "enabled")]
            pub enabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "configuration")]
            pub configuration: Option<SetTouchEmulationEnabledConfigurationOption>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StartScreencast {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "format")]
            pub format: Option<StartScreencastFormatOption>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "quality")]
            pub quality: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "maxWidth")]
            pub max_width: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "maxHeight")]
            pub max_height: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "everyNthFrame")]
            pub every_nth_frame: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopLoading(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Crash(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Close(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetWebLifecycleState {
            #[serde(rename = "state")]
            pub state: SetWebLifecycleStateStateOption,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopScreencast(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ProduceCompilationCache {
            #[serde(rename = "scripts")]
            pub scripts: Vec<CompilationCacheParams>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AddCompilationCache {
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(default)]
            #[serde(rename = "data")]
            pub data: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearCompilationCache(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetSPCTransactionMode {
            #[serde(rename = "mode")]
            pub mode: SetSPCTransactionModeModeOption,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GenerateTestReport {
            #[serde(default)]
            #[serde(rename = "message")]
            pub message: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "group")]
            pub group: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct WaitForDebugger(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetInterceptFileChooserDialog {
            #[serde(default)]
            #[serde(rename = "enabled")]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AddScriptToEvaluateOnLoadReturnObject {
            #[serde(rename = "identifier")]
            pub identifier: ScriptIdentifier,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AddScriptToEvaluateOnNewDocumentReturnObject {
            #[serde(rename = "identifier")]
            pub identifier: ScriptIdentifier,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BringToFrontReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CaptureScreenshotReturnObject {
            #[serde(default)]
            #[serde(rename = "data")]
            pub data: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CaptureSnapshotReturnObject {
            #[serde(default)]
            #[serde(rename = "data")]
            pub data: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDeviceMetricsOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDeviceOrientationOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearGeolocationOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CreateIsolatedWorldReturnObject {
            #[serde(rename = "executionContextId")]
            pub execution_context_id: Runtime::ExecutionContextId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteCookieReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetAppManifestReturnObject {
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(rename = "errors")]
            pub errors: Vec<AppManifestError>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "data")]
            pub data: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "parsed")]
            pub parsed: Option<AppManifestParsedProperties>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetInstallabilityErrorsReturnObject {
            #[serde(rename = "installabilityErrors")]
            pub installability_errors: Vec<InstallabilityError>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetManifestIconsReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "primaryIcon")]
            pub primary_icon: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetAppIdReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "appId")]
            pub app_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "recommendedId")]
            pub recommended_id: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetCookiesReturnObject {
            #[serde(rename = "cookies")]
            pub cookies: Network::Cookie,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetFrameTreeReturnObject {
            #[serde(rename = "frameTree")]
            pub frame_tree: FrameTree,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetLayoutMetricsReturnObject {
            #[serde(rename = "layoutViewport")]
            pub layout_viewport: LayoutViewport,
            #[serde(rename = "visualViewport")]
            pub visual_viewport: VisualViewport,
            #[serde(rename = "contentSize")]
            pub content_size: DOM::Rect,
            #[serde(rename = "cssLayoutViewport")]
            pub css_layout_viewport: LayoutViewport,
            #[serde(rename = "cssVisualViewport")]
            pub css_visual_viewport: VisualViewport,
            #[serde(rename = "cssContentSize")]
            pub css_content_size: DOM::Rect,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetNavigationHistoryReturnObject {
            #[serde(default)]
            #[serde(rename = "currentIndex")]
            pub current_index: JsUInt,
            #[serde(rename = "entries")]
            pub entries: Vec<NavigationEntry>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResetNavigationHistoryReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetResourceContentReturnObject {
            #[serde(default)]
            #[serde(rename = "content")]
            pub content: String,
            #[serde(default)]
            #[serde(rename = "base64Encoded")]
            pub base_64_encoded: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetResourceTreeReturnObject {
            #[serde(rename = "frameTree")]
            pub frame_tree: FrameResourceTree,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HandleJavaScriptDialogReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct NavigateReturnObject {
            #[serde(rename = "frameId")]
            pub frame_id: FrameId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "loaderId")]
            pub loader_id: Option<Network::LoaderId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "errorText")]
            pub error_text: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct NavigateToHistoryEntryReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PrintToPDFReturnObject {
            #[serde(default)]
            #[serde(rename = "data")]
            pub data: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "stream")]
            pub stream: Option<IO::StreamHandle>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReloadReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveScriptToEvaluateOnLoadReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveScriptToEvaluateOnNewDocumentReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ScreencastFrameAckReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SearchInResourceReturnObject {
            #[serde(rename = "result")]
            pub result: Debugger::SearchMatch,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAdBlockingEnabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBypassCSPReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetPermissionsPolicyStateReturnObject {
            #[serde(rename = "states")]
            pub states: Vec<PermissionsPolicyFeatureState>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetOriginTrialsReturnObject {
            #[serde(rename = "originTrials")]
            pub origin_trials: Vec<OriginTrial>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDeviceMetricsOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDeviceOrientationOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetFontFamiliesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetFontSizesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDocumentContentReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDownloadBehaviorReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetGeolocationOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetLifecycleEventsEnabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetTouchEmulationEnabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartScreencastReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopLoadingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CrashReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CloseReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetWebLifecycleStateReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopScreencastReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ProduceCompilationCacheReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddCompilationCacheReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearCompilationCacheReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetSPCTransactionModeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GenerateTestReportReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct WaitForDebuggerReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInterceptFileChooserDialogReturnObject {}
        impl Method for AddScriptToEvaluateOnLoad {
            const NAME: &'static str = "Page.addScriptToEvaluateOnLoad";
            type ReturnObject = AddScriptToEvaluateOnLoadReturnObject;
        }
        impl Method for AddScriptToEvaluateOnNewDocument {
            const NAME: &'static str = "Page.addScriptToEvaluateOnNewDocument";
            type ReturnObject = AddScriptToEvaluateOnNewDocumentReturnObject;
        }
        impl Method for BringToFront {
            const NAME: &'static str = "Page.bringToFront";
            type ReturnObject = BringToFrontReturnObject;
        }
        impl Method for CaptureScreenshot {
            const NAME: &'static str = "Page.captureScreenshot";
            type ReturnObject = CaptureScreenshotReturnObject;
        }
        impl Method for CaptureSnapshot {
            const NAME: &'static str = "Page.captureSnapshot";
            type ReturnObject = CaptureSnapshotReturnObject;
        }
        impl Method for ClearDeviceMetricsOverride {
            const NAME: &'static str = "Page.clearDeviceMetricsOverride";
            type ReturnObject = ClearDeviceMetricsOverrideReturnObject;
        }
        impl Method for ClearDeviceOrientationOverride {
            const NAME: &'static str = "Page.clearDeviceOrientationOverride";
            type ReturnObject = ClearDeviceOrientationOverrideReturnObject;
        }
        impl Method for ClearGeolocationOverride {
            const NAME: &'static str = "Page.clearGeolocationOverride";
            type ReturnObject = ClearGeolocationOverrideReturnObject;
        }
        impl Method for CreateIsolatedWorld {
            const NAME: &'static str = "Page.createIsolatedWorld";
            type ReturnObject = CreateIsolatedWorldReturnObject;
        }
        impl Method for DeleteCookie {
            const NAME: &'static str = "Page.deleteCookie";
            type ReturnObject = DeleteCookieReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Page.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Page.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetAppManifest {
            const NAME: &'static str = "Page.getAppManifest";
            type ReturnObject = GetAppManifestReturnObject;
        }
        impl Method for GetInstallabilityErrors {
            const NAME: &'static str = "Page.getInstallabilityErrors";
            type ReturnObject = GetInstallabilityErrorsReturnObject;
        }
        impl Method for GetManifestIcons {
            const NAME: &'static str = "Page.getManifestIcons";
            type ReturnObject = GetManifestIconsReturnObject;
        }
        impl Method for GetAppId {
            const NAME: &'static str = "Page.getAppId";
            type ReturnObject = GetAppIdReturnObject;
        }
        impl Method for GetCookies {
            const NAME: &'static str = "Page.getCookies";
            type ReturnObject = GetCookiesReturnObject;
        }
        impl Method for GetFrameTree {
            const NAME: &'static str = "Page.getFrameTree";
            type ReturnObject = GetFrameTreeReturnObject;
        }
        impl Method for GetLayoutMetrics {
            const NAME: &'static str = "Page.getLayoutMetrics";
            type ReturnObject = GetLayoutMetricsReturnObject;
        }
        impl Method for GetNavigationHistory {
            const NAME: &'static str = "Page.getNavigationHistory";
            type ReturnObject = GetNavigationHistoryReturnObject;
        }
        impl Method for ResetNavigationHistory {
            const NAME: &'static str = "Page.resetNavigationHistory";
            type ReturnObject = ResetNavigationHistoryReturnObject;
        }
        impl Method for GetResourceContent {
            const NAME: &'static str = "Page.getResourceContent";
            type ReturnObject = GetResourceContentReturnObject;
        }
        impl Method for GetResourceTree {
            const NAME: &'static str = "Page.getResourceTree";
            type ReturnObject = GetResourceTreeReturnObject;
        }
        impl Method for HandleJavaScriptDialog {
            const NAME: &'static str = "Page.handleJavaScriptDialog";
            type ReturnObject = HandleJavaScriptDialogReturnObject;
        }
        impl Method for Navigate {
            const NAME: &'static str = "Page.navigate";
            type ReturnObject = NavigateReturnObject;
        }
        impl Method for NavigateToHistoryEntry {
            const NAME: &'static str = "Page.navigateToHistoryEntry";
            type ReturnObject = NavigateToHistoryEntryReturnObject;
        }
        impl Method for PrintToPDF {
            const NAME: &'static str = "Page.printToPDF";
            type ReturnObject = PrintToPDFReturnObject;
        }
        impl Method for Reload {
            const NAME: &'static str = "Page.reload";
            type ReturnObject = ReloadReturnObject;
        }
        impl Method for RemoveScriptToEvaluateOnLoad {
            const NAME: &'static str = "Page.removeScriptToEvaluateOnLoad";
            type ReturnObject = RemoveScriptToEvaluateOnLoadReturnObject;
        }
        impl Method for RemoveScriptToEvaluateOnNewDocument {
            const NAME: &'static str = "Page.removeScriptToEvaluateOnNewDocument";
            type ReturnObject = RemoveScriptToEvaluateOnNewDocumentReturnObject;
        }
        impl Method for ScreencastFrameAck {
            const NAME: &'static str = "Page.screencastFrameAck";
            type ReturnObject = ScreencastFrameAckReturnObject;
        }
        impl Method for SearchInResource {
            const NAME: &'static str = "Page.searchInResource";
            type ReturnObject = SearchInResourceReturnObject;
        }
        impl Method for SetAdBlockingEnabled {
            const NAME: &'static str = "Page.setAdBlockingEnabled";
            type ReturnObject = SetAdBlockingEnabledReturnObject;
        }
        impl Method for SetBypassCSP {
            const NAME: &'static str = "Page.setBypassCSP";
            type ReturnObject = SetBypassCSPReturnObject;
        }
        impl Method for GetPermissionsPolicyState {
            const NAME: &'static str = "Page.getPermissionsPolicyState";
            type ReturnObject = GetPermissionsPolicyStateReturnObject;
        }
        impl Method for GetOriginTrials {
            const NAME: &'static str = "Page.getOriginTrials";
            type ReturnObject = GetOriginTrialsReturnObject;
        }
        impl Method for SetDeviceMetricsOverride {
            const NAME: &'static str = "Page.setDeviceMetricsOverride";
            type ReturnObject = SetDeviceMetricsOverrideReturnObject;
        }
        impl Method for SetDeviceOrientationOverride {
            const NAME: &'static str = "Page.setDeviceOrientationOverride";
            type ReturnObject = SetDeviceOrientationOverrideReturnObject;
        }
        impl Method for SetFontFamilies {
            const NAME: &'static str = "Page.setFontFamilies";
            type ReturnObject = SetFontFamiliesReturnObject;
        }
        impl Method for SetFontSizes {
            const NAME: &'static str = "Page.setFontSizes";
            type ReturnObject = SetFontSizesReturnObject;
        }
        impl Method for SetDocumentContent {
            const NAME: &'static str = "Page.setDocumentContent";
            type ReturnObject = SetDocumentContentReturnObject;
        }
        impl Method for SetDownloadBehavior {
            const NAME: &'static str = "Page.setDownloadBehavior";
            type ReturnObject = SetDownloadBehaviorReturnObject;
        }
        impl Method for SetGeolocationOverride {
            const NAME: &'static str = "Page.setGeolocationOverride";
            type ReturnObject = SetGeolocationOverrideReturnObject;
        }
        impl Method for SetLifecycleEventsEnabled {
            const NAME: &'static str = "Page.setLifecycleEventsEnabled";
            type ReturnObject = SetLifecycleEventsEnabledReturnObject;
        }
        impl Method for SetTouchEmulationEnabled {
            const NAME: &'static str = "Page.setTouchEmulationEnabled";
            type ReturnObject = SetTouchEmulationEnabledReturnObject;
        }
        impl Method for StartScreencast {
            const NAME: &'static str = "Page.startScreencast";
            type ReturnObject = StartScreencastReturnObject;
        }
        impl Method for StopLoading {
            const NAME: &'static str = "Page.stopLoading";
            type ReturnObject = StopLoadingReturnObject;
        }
        impl Method for Crash {
            const NAME: &'static str = "Page.crash";
            type ReturnObject = CrashReturnObject;
        }
        impl Method for Close {
            const NAME: &'static str = "Page.close";
            type ReturnObject = CloseReturnObject;
        }
        impl Method for SetWebLifecycleState {
            const NAME: &'static str = "Page.setWebLifecycleState";
            type ReturnObject = SetWebLifecycleStateReturnObject;
        }
        impl Method for StopScreencast {
            const NAME: &'static str = "Page.stopScreencast";
            type ReturnObject = StopScreencastReturnObject;
        }
        impl Method for ProduceCompilationCache {
            const NAME: &'static str = "Page.produceCompilationCache";
            type ReturnObject = ProduceCompilationCacheReturnObject;
        }
        impl Method for AddCompilationCache {
            const NAME: &'static str = "Page.addCompilationCache";
            type ReturnObject = AddCompilationCacheReturnObject;
        }
        impl Method for ClearCompilationCache {
            const NAME: &'static str = "Page.clearCompilationCache";
            type ReturnObject = ClearCompilationCacheReturnObject;
        }
        impl Method for SetSPCTransactionMode {
            const NAME: &'static str = "Page.setSPCTransactionMode";
            type ReturnObject = SetSPCTransactionModeReturnObject;
        }
        impl Method for GenerateTestReport {
            const NAME: &'static str = "Page.generateTestReport";
            type ReturnObject = GenerateTestReportReturnObject;
        }
        impl Method for WaitForDebugger {
            const NAME: &'static str = "Page.waitForDebugger";
            type ReturnObject = WaitForDebuggerReturnObject;
        }
        impl Method for SetInterceptFileChooserDialog {
            const NAME: &'static str = "Page.setInterceptFileChooserDialog";
            type ReturnObject = SetInterceptFileChooserDialogReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DomContentEventFiredEvent {
                pub params: DomContentEventFiredEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DomContentEventFiredEventParams {
                #[serde(rename = "timestamp")]
                pub timestamp: super::super::Network::MonotonicTime,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FileChooserOpenedEvent {
                pub params: FileChooserOpenedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FileChooserOpenedEventParams {
                #[serde(rename = "frameId")]
                pub frame_id: super::FrameId,
                #[serde(rename = "backendNodeId")]
                pub backend_node_id: super::super::DOM::BackendNodeId,
                #[serde(rename = "mode")]
                pub mode: super::FileChooserOpenedEventModeOption,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameAttachedEvent {
                pub params: FrameAttachedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameAttachedEventParams {
                #[serde(rename = "frameId")]
                pub frame_id: super::FrameId,
                #[serde(rename = "parentFrameId")]
                pub parent_frame_id: super::FrameId,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "stack")]
                pub stack: Option<super::super::Runtime::StackTrace>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameClearedScheduledNavigationEvent {
                pub params: FrameClearedScheduledNavigationEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameClearedScheduledNavigationEventParams {
                #[serde(rename = "frameId")]
                pub frame_id: super::FrameId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameDetachedEvent {
                pub params: FrameDetachedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameDetachedEventParams {
                #[serde(rename = "frameId")]
                pub frame_id: super::FrameId,
                #[serde(rename = "reason")]
                pub reason: super::FrameDetachedEventReasonOption,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameNavigatedEvent {
                pub params: FrameNavigatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameNavigatedEventParams {
                #[serde(rename = "frame")]
                pub frame: super::Frame,
                #[serde(rename = "type")]
                pub Type: super::NavigationType,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DocumentOpenedEvent {
                pub params: DocumentOpenedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DocumentOpenedEventParams {
                #[serde(rename = "frame")]
                pub frame: super::Frame,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct FrameResizedEvent(pub Option<serde_json::Value>);
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameRequestedNavigationEvent {
                pub params: FrameRequestedNavigationEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameRequestedNavigationEventParams {
                #[serde(rename = "frameId")]
                pub frame_id: super::FrameId,
                #[serde(rename = "reason")]
                pub reason: super::ClientNavigationReason,
                #[serde(default)]
                #[serde(rename = "url")]
                pub url: String,
                #[serde(rename = "disposition")]
                pub disposition: super::ClientNavigationDisposition,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameScheduledNavigationEvent {
                pub params: FrameScheduledNavigationEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameScheduledNavigationEventParams {
                #[serde(rename = "frameId")]
                pub frame_id: super::FrameId,
                #[serde(default)]
                #[serde(rename = "delay")]
                pub delay: JsFloat,
                #[serde(rename = "reason")]
                pub reason: super::ClientNavigationReason,
                #[serde(default)]
                #[serde(rename = "url")]
                pub url: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameStartedLoadingEvent {
                pub params: FrameStartedLoadingEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameStartedLoadingEventParams {
                #[serde(rename = "frameId")]
                pub frame_id: super::FrameId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameStoppedLoadingEvent {
                pub params: FrameStoppedLoadingEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameStoppedLoadingEventParams {
                #[serde(rename = "frameId")]
                pub frame_id: super::FrameId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DownloadWillBeginEvent {
                pub params: DownloadWillBeginEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DownloadWillBeginEventParams {
                #[serde(rename = "frameId")]
                pub frame_id: super::FrameId,
                #[serde(default)]
                #[serde(rename = "guid")]
                pub guid: String,
                #[serde(default)]
                #[serde(rename = "url")]
                pub url: String,
                #[serde(default)]
                #[serde(rename = "suggestedFilename")]
                pub suggested_filename: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DownloadProgressEvent {
                pub params: DownloadProgressEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DownloadProgressEventParams {
                #[serde(default)]
                #[serde(rename = "guid")]
                pub guid: String,
                #[serde(default)]
                #[serde(rename = "totalBytes")]
                pub total_bytes: JsFloat,
                #[serde(default)]
                #[serde(rename = "receivedBytes")]
                pub received_bytes: JsFloat,
                #[serde(rename = "state")]
                pub state: super::DownloadProgressEventStateOption,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct InterstitialHiddenEvent(pub Option<serde_json::Value>);
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct InterstitialShownEvent(pub Option<serde_json::Value>);
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct JavascriptDialogClosedEvent {
                pub params: JavascriptDialogClosedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct JavascriptDialogClosedEventParams {
                #[serde(default)]
                #[serde(rename = "result")]
                pub result: bool,
                #[serde(default)]
                #[serde(rename = "userInput")]
                pub user_input: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct JavascriptDialogOpeningEvent {
                pub params: JavascriptDialogOpeningEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct JavascriptDialogOpeningEventParams {
                #[serde(default)]
                #[serde(rename = "url")]
                pub url: String,
                #[serde(default)]
                #[serde(rename = "message")]
                pub message: String,
                #[serde(rename = "type")]
                pub Type: super::DialogType,
                #[serde(default)]
                #[serde(rename = "hasBrowserHandler")]
                pub has_browser_handler: bool,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "defaultPrompt")]
                pub default_prompt: Option<String>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LifecycleEventEvent {
                pub params: LifecycleEventEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LifecycleEventEventParams {
                #[serde(rename = "frameId")]
                pub frame_id: super::FrameId,
                #[serde(rename = "loaderId")]
                pub loader_id: super::super::Network::LoaderId,
                #[serde(default)]
                #[serde(rename = "name")]
                pub name: String,
                #[serde(rename = "timestamp")]
                pub timestamp: super::super::Network::MonotonicTime,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct BackForwardCacheNotUsedEvent {
                pub params: BackForwardCacheNotUsedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct BackForwardCacheNotUsedEventParams {
                #[serde(rename = "loaderId")]
                pub loader_id: super::super::Network::LoaderId,
                #[serde(rename = "frameId")]
                pub frame_id: super::FrameId,
                #[serde(rename = "notRestoredExplanations")]
                pub not_restored_explanations: Vec<super::BackForwardCacheNotRestoredExplanation>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LoadEventFiredEvent {
                pub params: LoadEventFiredEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LoadEventFiredEventParams {
                #[serde(rename = "timestamp")]
                pub timestamp: super::super::Network::MonotonicTime,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NavigatedWithinDocumentEvent {
                pub params: NavigatedWithinDocumentEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NavigatedWithinDocumentEventParams {
                #[serde(rename = "frameId")]
                pub frame_id: super::FrameId,
                #[serde(default)]
                #[serde(rename = "url")]
                pub url: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ScreencastFrameEvent {
                pub params: ScreencastFrameEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ScreencastFrameEventParams {
                #[serde(default)]
                #[serde(rename = "data")]
                pub data: String,
                #[serde(rename = "metadata")]
                pub metadata: super::ScreencastFrameMetadata,
                #[serde(default)]
                #[serde(rename = "sessionId")]
                pub session_id: JsUInt,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ScreencastVisibilityChangedEvent {
                pub params: ScreencastVisibilityChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ScreencastVisibilityChangedEventParams {
                #[serde(default)]
                #[serde(rename = "visible")]
                pub visible: bool,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WindowOpenEvent {
                pub params: WindowOpenEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WindowOpenEventParams {
                #[serde(default)]
                #[serde(rename = "url")]
                pub url: String,
                #[serde(default)]
                #[serde(rename = "windowName")]
                pub window_name: String,
                #[serde(default)]
                #[serde(rename = "windowFeatures")]
                pub window_features: Vec<String>,
                #[serde(default)]
                #[serde(rename = "userGesture")]
                pub user_gesture: bool,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct CompilationCacheProducedEvent {
                pub params: CompilationCacheProducedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct CompilationCacheProducedEventParams {
                #[serde(default)]
                #[serde(rename = "url")]
                pub url: String,
                #[serde(default)]
                #[serde(rename = "data")]
                pub data: String,
            }
        }
    }
    pub mod Performance {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum EnableTime_domainOption {
            #[serde(rename = "timeTicks")]
            TimeTicks,
            #[serde(rename = "threadTicks")]
            ThreadTicks,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SetTimeDomainTime_domainOption {
            #[serde(rename = "timeTicks")]
            TimeTicks,
            #[serde(rename = "threadTicks")]
            ThreadTicks,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Metric {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Enable {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "timeDomain")]
            pub time_domain: Option<EnableTime_domainOption>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetTimeDomain {
            #[serde(rename = "timeDomain")]
            pub time_domain: SetTimeDomainTime_domainOption,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetMetrics(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetTimeDomainReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetMetricsReturnObject {
            #[serde(rename = "metrics")]
            pub metrics: Vec<Metric>,
        }
        impl Method for Disable {
            const NAME: &'static str = "Performance.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Performance.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for SetTimeDomain {
            const NAME: &'static str = "Performance.setTimeDomain";
            type ReturnObject = SetTimeDomainReturnObject;
        }
        impl Method for GetMetrics {
            const NAME: &'static str = "Performance.getMetrics";
            type ReturnObject = GetMetricsReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct MetricsEvent {
                pub params: MetricsEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct MetricsEventParams {
                #[serde(rename = "metrics")]
                pub metrics: Vec<super::Metric>,
                #[serde(default)]
                #[serde(rename = "title")]
                pub title: String,
            }
        }
    }
    pub mod PerformanceTimeline {
        use super::types::*;
        use super::Network;
        use super::Page;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct LargestContentfulPaint {
            #[serde(rename = "renderTime")]
            pub render_time: Network::TimeSinceEpoch,
            #[serde(rename = "loadTime")]
            pub load_time: Network::TimeSinceEpoch,
            #[serde(default)]
            #[serde(rename = "size")]
            pub size: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "elementId")]
            pub element_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<DOM::BackendNodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct LayoutShiftAttribution {
            #[serde(rename = "previousRect")]
            pub previous_rect: DOM::Rect,
            #[serde(rename = "currentRect")]
            pub current_rect: DOM::Rect,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "nodeId")]
            pub node_id: Option<DOM::BackendNodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct LayoutShift {
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: JsFloat,
            #[serde(default)]
            #[serde(rename = "hadRecentInput")]
            pub had_recent_input: bool,
            #[serde(rename = "lastInputTime")]
            pub last_input_time: Network::TimeSinceEpoch,
            #[serde(rename = "sources")]
            pub sources: Vec<LayoutShiftAttribution>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TimelineEvent {
            #[serde(rename = "frameId")]
            pub frame_id: Page::FrameId,
            #[serde(default)]
            #[serde(rename = "type")]
            pub Type: String,
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(rename = "time")]
            pub time: Network::TimeSinceEpoch,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "duration")]
            pub duration: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "lcpDetails")]
            pub lcp_details: Option<LargestContentfulPaint>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "layoutShiftDetails")]
            pub layout_shift_details: Option<LayoutShift>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Enable {
            #[serde(default)]
            #[serde(rename = "eventTypes")]
            pub event_Types: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        impl Method for Enable {
            const NAME: &'static str = "PerformanceTimeline.enable";
            type ReturnObject = EnableReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TimelineEventAddedEvent {
                pub params: TimelineEventAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TimelineEventAddedEventParams {
                #[serde(rename = "event")]
                pub event: super::TimelineEvent,
            }
        }
    }
    pub mod Security {
        use super::types::*;
        use super::Network;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type CertificateId = JsUInt;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum MixedContentType {
            #[serde(rename = "blockable")]
            Blockable,
            #[serde(rename = "optionally-blockable")]
            OptionallyBlockable,
            #[serde(rename = "none")]
            None,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SecurityState {
            #[serde(rename = "unknown")]
            Unknown,
            #[serde(rename = "neutral")]
            Neutral,
            #[serde(rename = "insecure")]
            Insecure,
            #[serde(rename = "secure")]
            Secure,
            #[serde(rename = "info")]
            Info,
            #[serde(rename = "insecure-broken")]
            InsecureBroken,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SafetyTipStatus {
            #[serde(rename = "badReputation")]
            BadReputation,
            #[serde(rename = "lookalike")]
            Lookalike,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum CertificateErrorAction {
            #[serde(rename = "continue")]
            Continue,
            #[serde(rename = "cancel")]
            Cancel,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CertificateSecurityState {
            #[serde(default)]
            #[serde(rename = "protocol")]
            pub protocol: String,
            #[serde(default)]
            #[serde(rename = "keyExchange")]
            pub key_exchange: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "keyExchangeGroup")]
            pub key_exchange_group: Option<String>,
            #[serde(default)]
            #[serde(rename = "cipher")]
            pub cipher: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "mac")]
            pub mac: Option<String>,
            #[serde(default)]
            #[serde(rename = "certificate")]
            pub certificate: Vec<String>,
            #[serde(default)]
            #[serde(rename = "subjectName")]
            pub subject_name: String,
            #[serde(default)]
            #[serde(rename = "issuer")]
            pub issuer: String,
            #[serde(rename = "validFrom")]
            pub valid_from: Network::TimeSinceEpoch,
            #[serde(rename = "validTo")]
            pub valid_to: Network::TimeSinceEpoch,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "certificateNetworkError")]
            pub certificate_network_error: Option<String>,
            #[serde(default)]
            #[serde(rename = "certificateHasWeakSignature")]
            pub certificate_has_weak_signature: bool,
            #[serde(default)]
            #[serde(rename = "certificateHasSha1Signature")]
            pub certificate_has_sha_1_signature: bool,
            #[serde(default)]
            #[serde(rename = "modernSSL")]
            pub modern_ssl: bool,
            #[serde(default)]
            #[serde(rename = "obsoleteSslProtocol")]
            pub obsolete_ssl_protocol: bool,
            #[serde(default)]
            #[serde(rename = "obsoleteSslKeyExchange")]
            pub obsolete_ssl_key_exchange: bool,
            #[serde(default)]
            #[serde(rename = "obsoleteSslCipher")]
            pub obsolete_ssl_cipher: bool,
            #[serde(default)]
            #[serde(rename = "obsoleteSslSignature")]
            pub obsolete_ssl_signature: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SafetyTipInfo {
            #[serde(rename = "safetyTipStatus")]
            pub safety_tip_status: SafetyTipStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "safeUrl")]
            pub safe_url: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct VisibleSecurityState {
            #[serde(rename = "securityState")]
            pub security_state: SecurityState,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "certificateSecurityState")]
            pub certificate_security_state: Option<CertificateSecurityState>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "safetyTipInfo")]
            pub safety_tip_info: Option<SafetyTipInfo>,
            #[serde(default)]
            #[serde(rename = "securityStateIssueIds")]
            pub security_state_issue_ids: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SecurityStateExplanation {
            #[serde(rename = "securityState")]
            pub security_state: SecurityState,
            #[serde(default)]
            #[serde(rename = "title")]
            pub title: String,
            #[serde(default)]
            #[serde(rename = "summary")]
            pub summary: String,
            #[serde(default)]
            #[serde(rename = "description")]
            pub description: String,
            #[serde(rename = "mixedContentType")]
            pub mixed_content_Type: MixedContentType,
            #[serde(default)]
            #[serde(rename = "certificate")]
            pub certificate: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "recommendations")]
            pub recommendations: Option<Vec<String>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct InsecureContentStatus {
            #[serde(default)]
            #[serde(rename = "ranMixedContent")]
            pub ran_mixed_content: bool,
            #[serde(default)]
            #[serde(rename = "displayedMixedContent")]
            pub displayed_mixed_content: bool,
            #[serde(default)]
            #[serde(rename = "containedMixedForm")]
            pub contained_mixed_form: bool,
            #[serde(default)]
            #[serde(rename = "ranContentWithCertErrors")]
            pub ran_content_with_cert_errors: bool,
            #[serde(default)]
            #[serde(rename = "displayedContentWithCertErrors")]
            pub displayed_content_with_cert_errors: bool,
            #[serde(rename = "ranInsecureContentStyle")]
            pub ran_insecure_content_style: SecurityState,
            #[serde(rename = "displayedInsecureContentStyle")]
            pub displayed_insecure_content_style: SecurityState,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetIgnoreCertificateErrors {
            #[serde(default)]
            #[serde(rename = "ignore")]
            pub ignore: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct HandleCertificateError {
            #[serde(default)]
            #[serde(rename = "eventId")]
            pub event_id: JsUInt,
            #[serde(rename = "action")]
            pub action: CertificateErrorAction,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetOverrideCertificateErrors {
            #[serde(default)]
            #[serde(rename = "override")]
            pub Override: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetIgnoreCertificateErrorsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HandleCertificateErrorReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetOverrideCertificateErrorsReturnObject {}
        impl Method for Disable {
            const NAME: &'static str = "Security.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Security.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for SetIgnoreCertificateErrors {
            const NAME: &'static str = "Security.setIgnoreCertificateErrors";
            type ReturnObject = SetIgnoreCertificateErrorsReturnObject;
        }
        impl Method for HandleCertificateError {
            const NAME: &'static str = "Security.handleCertificateError";
            type ReturnObject = HandleCertificateErrorReturnObject;
        }
        impl Method for SetOverrideCertificateErrors {
            const NAME: &'static str = "Security.setOverrideCertificateErrors";
            type ReturnObject = SetOverrideCertificateErrorsReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct CertificateErrorEvent {
                pub params: CertificateErrorEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct CertificateErrorEventParams {
                #[serde(default)]
                #[serde(rename = "eventId")]
                pub event_id: JsUInt,
                #[serde(default)]
                #[serde(rename = "errorType")]
                pub error_Type: String,
                #[serde(default)]
                #[serde(rename = "requestURL")]
                pub request_url: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct VisibleSecurityStateChangedEvent {
                pub params: VisibleSecurityStateChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct VisibleSecurityStateChangedEventParams {
                #[serde(rename = "visibleSecurityState")]
                pub visible_security_state: super::VisibleSecurityState,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SecurityStateChangedEvent {
                pub params: SecurityStateChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SecurityStateChangedEventParams {
                #[serde(rename = "securityState")]
                pub security_state: super::SecurityState,
                #[serde(default)]
                #[serde(rename = "schemeIsCryptographic")]
                pub scheme_is_cryptographic: bool,
                #[serde(rename = "explanations")]
                pub explanations: Vec<super::SecurityStateExplanation>,
                #[serde(rename = "insecureContentStatus")]
                pub insecure_content_status: super::InsecureContentStatus,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "summary")]
                pub summary: Option<String>,
            }
        }
    }
    pub mod ServiceWorker {
        use super::types::*;
        use super::Target;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type RegistrationID = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ServiceWorkerVersionRunningStatus {
            #[serde(rename = "stopped")]
            Stopped,
            #[serde(rename = "starting")]
            Starting,
            #[serde(rename = "running")]
            Running,
            #[serde(rename = "stopping")]
            Stopping,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ServiceWorkerVersionStatus {
            #[serde(rename = "new")]
            New,
            #[serde(rename = "installing")]
            Installing,
            #[serde(rename = "installed")]
            Installed,
            #[serde(rename = "activating")]
            Activating,
            #[serde(rename = "activated")]
            Activated,
            #[serde(rename = "redundant")]
            Redundant,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ServiceWorkerRegistration {
            #[serde(rename = "registrationId")]
            pub registration_id: RegistrationID,
            #[serde(default)]
            #[serde(rename = "scopeURL")]
            pub scope_url: String,
            #[serde(default)]
            #[serde(rename = "isDeleted")]
            pub is_deleted: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ServiceWorkerVersion {
            #[serde(default)]
            #[serde(rename = "versionId")]
            pub version_id: String,
            #[serde(rename = "registrationId")]
            pub registration_id: RegistrationID,
            #[serde(default)]
            #[serde(rename = "scriptURL")]
            pub script_url: String,
            #[serde(rename = "runningStatus")]
            pub running_status: ServiceWorkerVersionRunningStatus,
            #[serde(rename = "status")]
            pub status: ServiceWorkerVersionStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "scriptLastModified")]
            pub script_last_modified: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "scriptResponseTime")]
            pub script_response_time: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "controlledClients")]
            pub controlled_clients: Option<Vec<Target::TargetID>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "targetId")]
            pub target_id: Option<Target::TargetID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ServiceWorkerErrorMessage {
            #[serde(default)]
            #[serde(rename = "errorMessage")]
            pub error_message: String,
            #[serde(rename = "registrationId")]
            pub registration_id: RegistrationID,
            #[serde(default)]
            #[serde(rename = "versionId")]
            pub version_id: String,
            #[serde(default)]
            #[serde(rename = "sourceURL")]
            pub source_url: String,
            #[serde(default)]
            #[serde(rename = "lineNumber")]
            pub line_number: JsUInt,
            #[serde(default)]
            #[serde(rename = "columnNumber")]
            pub column_number: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DeliverPushMessage {
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: String,
            #[serde(rename = "registrationId")]
            pub registration_id: RegistrationID,
            #[serde(default)]
            #[serde(rename = "data")]
            pub data: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DispatchSyncEvent {
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: String,
            #[serde(rename = "registrationId")]
            pub registration_id: RegistrationID,
            #[serde(default)]
            #[serde(rename = "tag")]
            pub tag: String,
            #[serde(default)]
            #[serde(rename = "lastChance")]
            pub last_chance: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DispatchPeriodicSyncEvent {
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: String,
            #[serde(rename = "registrationId")]
            pub registration_id: RegistrationID,
            #[serde(default)]
            #[serde(rename = "tag")]
            pub tag: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct InspectWorker {
            #[serde(default)]
            #[serde(rename = "versionId")]
            pub version_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetForceUpdateOnPageLoad {
            #[serde(default)]
            #[serde(rename = "forceUpdateOnPageLoad")]
            pub force_update_on_page_load: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SkipWaiting {
            #[serde(default)]
            #[serde(rename = "scopeURL")]
            pub scope_url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StartWorker {
            #[serde(default)]
            #[serde(rename = "scopeURL")]
            pub scope_url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopAllWorkers(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct StopWorker {
            #[serde(default)]
            #[serde(rename = "versionId")]
            pub version_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Unregister {
            #[serde(default)]
            #[serde(rename = "scopeURL")]
            pub scope_url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct UpdateRegistration {
            #[serde(default)]
            #[serde(rename = "scopeURL")]
            pub scope_url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeliverPushMessageReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchSyncEventReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchPeriodicSyncEventReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct InspectWorkerReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetForceUpdateOnPageLoadReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SkipWaitingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartWorkerReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopAllWorkersReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopWorkerReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UnregisterReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UpdateRegistrationReturnObject {}
        impl Method for DeliverPushMessage {
            const NAME: &'static str = "ServiceWorker.deliverPushMessage";
            type ReturnObject = DeliverPushMessageReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "ServiceWorker.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for DispatchSyncEvent {
            const NAME: &'static str = "ServiceWorker.dispatchSyncEvent";
            type ReturnObject = DispatchSyncEventReturnObject;
        }
        impl Method for DispatchPeriodicSyncEvent {
            const NAME: &'static str = "ServiceWorker.dispatchPeriodicSyncEvent";
            type ReturnObject = DispatchPeriodicSyncEventReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "ServiceWorker.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for InspectWorker {
            const NAME: &'static str = "ServiceWorker.inspectWorker";
            type ReturnObject = InspectWorkerReturnObject;
        }
        impl Method for SetForceUpdateOnPageLoad {
            const NAME: &'static str = "ServiceWorker.setForceUpdateOnPageLoad";
            type ReturnObject = SetForceUpdateOnPageLoadReturnObject;
        }
        impl Method for SkipWaiting {
            const NAME: &'static str = "ServiceWorker.skipWaiting";
            type ReturnObject = SkipWaitingReturnObject;
        }
        impl Method for StartWorker {
            const NAME: &'static str = "ServiceWorker.startWorker";
            type ReturnObject = StartWorkerReturnObject;
        }
        impl Method for StopAllWorkers {
            const NAME: &'static str = "ServiceWorker.stopAllWorkers";
            type ReturnObject = StopAllWorkersReturnObject;
        }
        impl Method for StopWorker {
            const NAME: &'static str = "ServiceWorker.stopWorker";
            type ReturnObject = StopWorkerReturnObject;
        }
        impl Method for Unregister {
            const NAME: &'static str = "ServiceWorker.unregister";
            type ReturnObject = UnregisterReturnObject;
        }
        impl Method for UpdateRegistration {
            const NAME: &'static str = "ServiceWorker.updateRegistration";
            type ReturnObject = UpdateRegistrationReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WorkerErrorReportedEvent {
                pub params: WorkerErrorReportedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WorkerErrorReportedEventParams {
                #[serde(rename = "errorMessage")]
                pub error_message: super::ServiceWorkerErrorMessage,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WorkerRegistrationUpdatedEvent {
                pub params: WorkerRegistrationUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WorkerRegistrationUpdatedEventParams {
                #[serde(rename = "registrations")]
                pub registrations: Vec<super::ServiceWorkerRegistration>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WorkerVersionUpdatedEvent {
                pub params: WorkerVersionUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WorkerVersionUpdatedEventParams {
                #[serde(rename = "versions")]
                pub versions: Vec<super::ServiceWorkerVersion>,
            }
        }
    }
    pub mod Storage {
        use super::types::*;
        use super::Browser;
        use super::Network;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum StorageType {
            #[serde(rename = "appcache")]
            Appcache,
            #[serde(rename = "cookies")]
            Cookies,
            #[serde(rename = "file_systems")]
            FileSystems,
            #[serde(rename = "indexeddb")]
            Indexeddb,
            #[serde(rename = "local_storage")]
            LocalStorage,
            #[serde(rename = "shader_cache")]
            ShaderCache,
            #[serde(rename = "websql")]
            Websql,
            #[serde(rename = "service_workers")]
            ServiceWorkers,
            #[serde(rename = "cache_storage")]
            CacheStorage,
            #[serde(rename = "all")]
            All,
            #[serde(rename = "other")]
            Other,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct UsageForType {
            #[serde(rename = "storageType")]
            pub storage_Type: StorageType,
            #[serde(default)]
            #[serde(rename = "usage")]
            pub usage: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TrustTokens {
            #[serde(default)]
            #[serde(rename = "issuerOrigin")]
            pub issuer_origin: String,
            #[serde(default)]
            #[serde(rename = "count")]
            pub count: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ClearDataForOrigin {
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: String,
            #[serde(default)]
            #[serde(rename = "storageTypes")]
            pub storage_Types: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetCookies {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "browserContextId")]
            pub browser_context_id: Option<Browser::BrowserContextID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetCookies {
            #[serde(rename = "cookies")]
            pub cookies: Network::CookieParam,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "browserContextId")]
            pub browser_context_id: Option<Browser::BrowserContextID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ClearCookies {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "browserContextId")]
            pub browser_context_id: Option<Browser::BrowserContextID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetUsageAndQuota {
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct OverrideQuotaForOrigin {
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "quotaSize")]
            pub quota_size: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TrackCacheStorageForOrigin {
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TrackIndexedDBForOrigin {
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct UntrackCacheStorageForOrigin {
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct UntrackIndexedDBForOrigin {
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetTrustTokens(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ClearTrustTokens {
            #[serde(default)]
            #[serde(rename = "issuerOrigin")]
            pub issuer_origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDataForOriginReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetCookiesReturnObject {
            #[serde(rename = "cookies")]
            pub cookies: Network::Cookie,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetCookiesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearCookiesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetUsageAndQuotaReturnObject {
            #[serde(default)]
            #[serde(rename = "usage")]
            pub usage: JsFloat,
            #[serde(default)]
            #[serde(rename = "quota")]
            pub quota: JsFloat,
            #[serde(default)]
            #[serde(rename = "overrideActive")]
            pub override_active: bool,
            #[serde(rename = "usageBreakdown")]
            pub usage_breakdown: Vec<UsageForType>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct OverrideQuotaForOriginReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TrackCacheStorageForOriginReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TrackIndexedDBForOriginReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UntrackCacheStorageForOriginReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UntrackIndexedDBForOriginReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetTrustTokensReturnObject {
            #[serde(rename = "tokens")]
            pub tokens: Vec<TrustTokens>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ClearTrustTokensReturnObject {
            #[serde(default)]
            #[serde(rename = "didDeleteTokens")]
            pub did_delete_tokens: bool,
        }
        impl Method for ClearDataForOrigin {
            const NAME: &'static str = "Storage.clearDataForOrigin";
            type ReturnObject = ClearDataForOriginReturnObject;
        }
        impl Method for GetCookies {
            const NAME: &'static str = "Storage.getCookies";
            type ReturnObject = GetCookiesReturnObject;
        }
        impl Method for SetCookies {
            const NAME: &'static str = "Storage.setCookies";
            type ReturnObject = SetCookiesReturnObject;
        }
        impl Method for ClearCookies {
            const NAME: &'static str = "Storage.clearCookies";
            type ReturnObject = ClearCookiesReturnObject;
        }
        impl Method for GetUsageAndQuota {
            const NAME: &'static str = "Storage.getUsageAndQuota";
            type ReturnObject = GetUsageAndQuotaReturnObject;
        }
        impl Method for OverrideQuotaForOrigin {
            const NAME: &'static str = "Storage.overrideQuotaForOrigin";
            type ReturnObject = OverrideQuotaForOriginReturnObject;
        }
        impl Method for TrackCacheStorageForOrigin {
            const NAME: &'static str = "Storage.trackCacheStorageForOrigin";
            type ReturnObject = TrackCacheStorageForOriginReturnObject;
        }
        impl Method for TrackIndexedDBForOrigin {
            const NAME: &'static str = "Storage.trackIndexedDBForOrigin";
            type ReturnObject = TrackIndexedDBForOriginReturnObject;
        }
        impl Method for UntrackCacheStorageForOrigin {
            const NAME: &'static str = "Storage.untrackCacheStorageForOrigin";
            type ReturnObject = UntrackCacheStorageForOriginReturnObject;
        }
        impl Method for UntrackIndexedDBForOrigin {
            const NAME: &'static str = "Storage.untrackIndexedDBForOrigin";
            type ReturnObject = UntrackIndexedDBForOriginReturnObject;
        }
        impl Method for GetTrustTokens {
            const NAME: &'static str = "Storage.getTrustTokens";
            type ReturnObject = GetTrustTokensReturnObject;
        }
        impl Method for ClearTrustTokens {
            const NAME: &'static str = "Storage.clearTrustTokens";
            type ReturnObject = ClearTrustTokensReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct CacheStorageContentUpdatedEvent {
                pub params: CacheStorageContentUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct CacheStorageContentUpdatedEventParams {
                #[serde(default)]
                #[serde(rename = "origin")]
                pub origin: String,
                #[serde(default)]
                #[serde(rename = "cacheName")]
                pub cache_name: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct CacheStorageListUpdatedEvent {
                pub params: CacheStorageListUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct CacheStorageListUpdatedEventParams {
                #[serde(default)]
                #[serde(rename = "origin")]
                pub origin: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct IndexedDBContentUpdatedEvent {
                pub params: IndexedDBContentUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct IndexedDBContentUpdatedEventParams {
                #[serde(default)]
                #[serde(rename = "origin")]
                pub origin: String,
                #[serde(default)]
                #[serde(rename = "databaseName")]
                pub database_name: String,
                #[serde(default)]
                #[serde(rename = "objectStoreName")]
                pub object_store_name: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct IndexedDBListUpdatedEvent {
                pub params: IndexedDBListUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct IndexedDBListUpdatedEventParams {
                #[serde(default)]
                #[serde(rename = "origin")]
                pub origin: String,
            }
        }
    }
    pub mod SystemInfo {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum SubsamplingFormat {
            #[serde(rename = "yuv420")]
            Yuv420,
            #[serde(rename = "yuv422")]
            Yuv422,
            #[serde(rename = "yuv444")]
            Yuv444,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ImageType {
            #[serde(rename = "jpeg")]
            Jpeg,
            #[serde(rename = "webp")]
            Webp,
            #[serde(rename = "unknown")]
            Unknown,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GPUDevice {
            #[serde(default)]
            #[serde(rename = "vendorId")]
            pub vendor_id: JsFloat,
            #[serde(default)]
            #[serde(rename = "deviceId")]
            pub device_id: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "subSysId")]
            pub sub_sys_id: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "revision")]
            pub revision: Option<JsFloat>,
            #[serde(default)]
            #[serde(rename = "vendorString")]
            pub vendor_string: String,
            #[serde(default)]
            #[serde(rename = "deviceString")]
            pub device_string: String,
            #[serde(default)]
            #[serde(rename = "driverVendor")]
            pub driver_vendor: String,
            #[serde(default)]
            #[serde(rename = "driverVersion")]
            pub driver_version: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Size {
            #[serde(default)]
            #[serde(rename = "width")]
            pub width: JsUInt,
            #[serde(default)]
            #[serde(rename = "height")]
            pub height: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct VideoDecodeAcceleratorCapability {
            #[serde(default)]
            #[serde(rename = "profile")]
            pub profile: String,
            #[serde(rename = "maxResolution")]
            pub max_resolution: Size,
            #[serde(rename = "minResolution")]
            pub min_resolution: Size,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct VideoEncodeAcceleratorCapability {
            #[serde(default)]
            #[serde(rename = "profile")]
            pub profile: String,
            #[serde(rename = "maxResolution")]
            pub max_resolution: Size,
            #[serde(default)]
            #[serde(rename = "maxFramerateNumerator")]
            pub max_framerate_numerator: JsUInt,
            #[serde(default)]
            #[serde(rename = "maxFramerateDenominator")]
            pub max_framerate_denominator: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ImageDecodeAcceleratorCapability {
            #[serde(rename = "imageType")]
            pub image_Type: ImageType,
            #[serde(rename = "maxDimensions")]
            pub max_dimensions: Size,
            #[serde(rename = "minDimensions")]
            pub min_dimensions: Size,
            #[serde(rename = "subsamplings")]
            pub subsamplings: Vec<SubsamplingFormat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GPUInfo {
            #[serde(rename = "devices")]
            pub devices: Vec<GPUDevice>,
            #[serde(default)]
            #[serde(rename = "driverBugWorkarounds")]
            pub driver_bug_workarounds: Vec<String>,
            #[serde(rename = "videoDecoding")]
            pub video_decoding: Vec<VideoDecodeAcceleratorCapability>,
            #[serde(rename = "videoEncoding")]
            pub video_encoding: Vec<VideoEncodeAcceleratorCapability>,
            #[serde(rename = "imageDecoding")]
            pub image_decoding: Vec<ImageDecodeAcceleratorCapability>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ProcessInfo {
            #[serde(default)]
            #[serde(rename = "type")]
            pub Type: String,
            #[serde(default)]
            #[serde(rename = "id")]
            pub id: JsUInt,
            #[serde(default)]
            #[serde(rename = "cpuTime")]
            pub cpu_time: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetInfo(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetProcessInfo(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetInfoReturnObject {
            #[serde(rename = "gpu")]
            pub gpu: GPUInfo,
            #[serde(default)]
            #[serde(rename = "modelName")]
            pub model_name: String,
            #[serde(default)]
            #[serde(rename = "modelVersion")]
            pub model_version: String,
            #[serde(default)]
            #[serde(rename = "commandLine")]
            pub command_line: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetProcessInfoReturnObject {
            #[serde(rename = "processInfo")]
            pub process_info: Vec<ProcessInfo>,
        }
        impl Method for GetInfo {
            const NAME: &'static str = "SystemInfo.getInfo";
            type ReturnObject = GetInfoReturnObject;
        }
        impl Method for GetProcessInfo {
            const NAME: &'static str = "SystemInfo.getProcessInfo";
            type ReturnObject = GetProcessInfoReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod Target {
        use super::types::*;
        use super::Browser;
        use super::Page;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type TargetID = String;
        pub type SessionID = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TargetInfo {
            #[serde(rename = "targetId")]
            pub target_id: TargetID,
            #[serde(default)]
            #[serde(rename = "type")]
            pub Type: String,
            #[serde(default)]
            #[serde(rename = "title")]
            pub title: String,
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(default)]
            #[serde(rename = "attached")]
            pub attached: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "openerId")]
            pub opener_id: Option<TargetID>,
            #[serde(default)]
            #[serde(rename = "canAccessOpener")]
            pub can_access_opener: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "openerFrameId")]
            pub opener_frame_id: Option<Page::FrameId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "browserContextId")]
            pub browser_context_id: Option<Browser::BrowserContextID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RemoteLocation {
            #[serde(default)]
            #[serde(rename = "host")]
            pub host: String,
            #[serde(default)]
            #[serde(rename = "port")]
            pub port: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ActivateTarget {
            #[serde(rename = "targetId")]
            pub target_id: TargetID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AttachToTarget {
            #[serde(rename = "targetId")]
            pub target_id: TargetID,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "flatten")]
            pub flatten: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AttachToBrowserTarget(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CloseTarget {
            #[serde(rename = "targetId")]
            pub target_id: TargetID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ExposeDevToolsProtocol {
            #[serde(rename = "targetId")]
            pub target_id: TargetID,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "bindingName")]
            pub binding_name: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CreateBrowserContext {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "disposeOnDetach")]
            pub dispose_on_detach: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "proxyServer")]
            pub proxy_server: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "proxyBypassList")]
            pub proxy_bypass_list: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "originsWithUniversalNetworkAccess")]
            pub origins_with_universal_network_access: Option<Vec<String>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetBrowserContexts(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CreateTarget {
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "width")]
            pub width: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "height")]
            pub height: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "browserContextId")]
            pub browser_context_id: Option<Browser::BrowserContextID>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "enableBeginFrameControl")]
            pub enable_begin_frame_control: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "newWindow")]
            pub new_window: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "background")]
            pub background: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DetachFromTarget {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "sessionId")]
            pub session_id: Option<SessionID>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "targetId")]
            pub target_id: Option<TargetID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct DisposeBrowserContext {
            #[serde(rename = "browserContextId")]
            pub browser_context_id: Browser::BrowserContextID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetTargetInfo {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "targetId")]
            pub target_id: Option<TargetID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetTargets(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SendMessageToTarget {
            #[serde(default)]
            #[serde(rename = "message")]
            pub message: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "sessionId")]
            pub session_id: Option<SessionID>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "targetId")]
            pub target_id: Option<TargetID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetAutoAttach {
            #[serde(default)]
            #[serde(rename = "autoAttach")]
            pub auto_attach: bool,
            #[serde(default)]
            #[serde(rename = "waitForDebuggerOnStart")]
            pub wait_for_debugger_on_start: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "flatten")]
            pub flatten: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AutoAttachRelated {
            #[serde(rename = "targetId")]
            pub target_id: TargetID,
            #[serde(default)]
            #[serde(rename = "waitForDebuggerOnStart")]
            pub wait_for_debugger_on_start: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetDiscoverTargets {
            #[serde(default)]
            #[serde(rename = "discover")]
            pub discover: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetRemoteLocations {
            #[serde(rename = "locations")]
            pub locations: Vec<RemoteLocation>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ActivateTargetReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AttachToTargetReturnObject {
            #[serde(rename = "sessionId")]
            pub session_id: SessionID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AttachToBrowserTargetReturnObject {
            #[serde(rename = "sessionId")]
            pub session_id: SessionID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CloseTargetReturnObject {
            #[serde(default)]
            #[serde(rename = "success")]
            pub success: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ExposeDevToolsProtocolReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CreateBrowserContextReturnObject {
            #[serde(rename = "browserContextId")]
            pub browser_context_id: Browser::BrowserContextID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetBrowserContextsReturnObject {
            #[serde(rename = "browserContextIds")]
            pub browser_context_ids: Browser::BrowserContextID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct CreateTargetReturnObject {
            #[serde(rename = "targetId")]
            pub target_id: TargetID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DetachFromTargetReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisposeBrowserContextReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetTargetInfoReturnObject {
            #[serde(rename = "targetInfo")]
            pub target_info: TargetInfo,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetTargetsReturnObject {
            #[serde(rename = "targetInfos")]
            pub target_infos: Vec<TargetInfo>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SendMessageToTargetReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAutoAttachReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AutoAttachRelatedReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDiscoverTargetsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetRemoteLocationsReturnObject {}
        impl Method for ActivateTarget {
            const NAME: &'static str = "Target.activateTarget";
            type ReturnObject = ActivateTargetReturnObject;
        }
        impl Method for AttachToTarget {
            const NAME: &'static str = "Target.attachToTarget";
            type ReturnObject = AttachToTargetReturnObject;
        }
        impl Method for AttachToBrowserTarget {
            const NAME: &'static str = "Target.attachToBrowserTarget";
            type ReturnObject = AttachToBrowserTargetReturnObject;
        }
        impl Method for CloseTarget {
            const NAME: &'static str = "Target.closeTarget";
            type ReturnObject = CloseTargetReturnObject;
        }
        impl Method for ExposeDevToolsProtocol {
            const NAME: &'static str = "Target.exposeDevToolsProtocol";
            type ReturnObject = ExposeDevToolsProtocolReturnObject;
        }
        impl Method for CreateBrowserContext {
            const NAME: &'static str = "Target.createBrowserContext";
            type ReturnObject = CreateBrowserContextReturnObject;
        }
        impl Method for GetBrowserContexts {
            const NAME: &'static str = "Target.getBrowserContexts";
            type ReturnObject = GetBrowserContextsReturnObject;
        }
        impl Method for CreateTarget {
            const NAME: &'static str = "Target.createTarget";
            type ReturnObject = CreateTargetReturnObject;
        }
        impl Method for DetachFromTarget {
            const NAME: &'static str = "Target.detachFromTarget";
            type ReturnObject = DetachFromTargetReturnObject;
        }
        impl Method for DisposeBrowserContext {
            const NAME: &'static str = "Target.disposeBrowserContext";
            type ReturnObject = DisposeBrowserContextReturnObject;
        }
        impl Method for GetTargetInfo {
            const NAME: &'static str = "Target.getTargetInfo";
            type ReturnObject = GetTargetInfoReturnObject;
        }
        impl Method for GetTargets {
            const NAME: &'static str = "Target.getTargets";
            type ReturnObject = GetTargetsReturnObject;
        }
        impl Method for SendMessageToTarget {
            const NAME: &'static str = "Target.sendMessageToTarget";
            type ReturnObject = SendMessageToTargetReturnObject;
        }
        impl Method for SetAutoAttach {
            const NAME: &'static str = "Target.setAutoAttach";
            type ReturnObject = SetAutoAttachReturnObject;
        }
        impl Method for AutoAttachRelated {
            const NAME: &'static str = "Target.autoAttachRelated";
            type ReturnObject = AutoAttachRelatedReturnObject;
        }
        impl Method for SetDiscoverTargets {
            const NAME: &'static str = "Target.setDiscoverTargets";
            type ReturnObject = SetDiscoverTargetsReturnObject;
        }
        impl Method for SetRemoteLocations {
            const NAME: &'static str = "Target.setRemoteLocations";
            type ReturnObject = SetRemoteLocationsReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AttachedToTargetEvent {
                pub params: AttachedToTargetEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AttachedToTargetEventParams {
                #[serde(rename = "sessionId")]
                pub session_id: super::SessionID,
                #[serde(rename = "targetInfo")]
                pub target_info: super::TargetInfo,
                #[serde(default)]
                #[serde(rename = "waitingForDebugger")]
                pub waiting_for_debugger: bool,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DetachedFromTargetEvent {
                pub params: DetachedFromTargetEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DetachedFromTargetEventParams {
                #[serde(rename = "sessionId")]
                pub session_id: super::SessionID,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "targetId")]
                pub target_id: Option<super::TargetID>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ReceivedMessageFromTargetEvent {
                pub params: ReceivedMessageFromTargetEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ReceivedMessageFromTargetEventParams {
                #[serde(rename = "sessionId")]
                pub session_id: super::SessionID,
                #[serde(default)]
                #[serde(rename = "message")]
                pub message: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "targetId")]
                pub target_id: Option<super::TargetID>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TargetCreatedEvent {
                pub params: TargetCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TargetCreatedEventParams {
                #[serde(rename = "targetInfo")]
                pub target_info: super::TargetInfo,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TargetDestroyedEvent {
                pub params: TargetDestroyedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TargetDestroyedEventParams {
                #[serde(rename = "targetId")]
                pub target_id: super::TargetID,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TargetCrashedEvent {
                pub params: TargetCrashedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TargetCrashedEventParams {
                #[serde(rename = "targetId")]
                pub target_id: super::TargetID,
                #[serde(default)]
                #[serde(rename = "status")]
                pub status: String,
                #[serde(default)]
                #[serde(rename = "errorCode")]
                pub error_code: JsUInt,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TargetInfoChangedEvent {
                pub params: TargetInfoChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TargetInfoChangedEventParams {
                #[serde(rename = "targetInfo")]
                pub target_info: super::TargetInfo,
            }
        }
    }
    pub mod Tethering {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Bind {
            #[serde(default)]
            #[serde(rename = "port")]
            pub port: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Unbind {
            #[serde(default)]
            #[serde(rename = "port")]
            pub port: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BindReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UnbindReturnObject {}
        impl Method for Bind {
            const NAME: &'static str = "Tethering.bind";
            type ReturnObject = BindReturnObject;
        }
        impl Method for Unbind {
            const NAME: &'static str = "Tethering.unbind";
            type ReturnObject = UnbindReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AcceptedEvent {
                pub params: AcceptedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AcceptedEventParams {
                #[serde(default)]
                #[serde(rename = "port")]
                pub port: JsUInt,
                #[serde(default)]
                #[serde(rename = "connectionId")]
                pub connection_id: String,
            }
        }
    }
    pub mod Tracing {
        use super::types::*;
        use super::IO;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum TraceConfigRecordMode {
            #[serde(rename = "recordUntilFull")]
            RecordUntilFull,
            #[serde(rename = "recordContinuously")]
            RecordContinuously,
            #[serde(rename = "recordAsMuchAsPossible")]
            RecordAsMuchAsPossible,
            #[serde(rename = "echoToConsole")]
            EchoToConsole,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum StreamFormat {
            #[serde(rename = "json")]
            Json,
            #[serde(rename = "proto")]
            Proto,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum StreamCompression {
            #[serde(rename = "none")]
            None,
            #[serde(rename = "gzip")]
            Gzip,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum MemoryDumpLevelOfDetail {
            #[serde(rename = "background")]
            Background,
            #[serde(rename = "light")]
            Light,
            #[serde(rename = "detailed")]
            Detailed,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum TracingBackend {
            #[serde(rename = "auto")]
            Auto,
            #[serde(rename = "chrome")]
            Chrome,
            #[serde(rename = "system")]
            System,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum StartTransfer_modeOption {
            #[serde(rename = "ReportEvents")]
            ReportEvents,
            #[serde(rename = "ReturnAsStream")]
            ReturnAsStream,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct MemoryDumpConfig(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TraceConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "recordMode")]
            pub record_mode: Option<TraceConfigRecordMode>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "enableSampling")]
            pub enable_sampling: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "enableSystrace")]
            pub enable_systrace: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "enableArgumentFilter")]
            pub enable_argument_filter: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "includedCategories")]
            pub included_categories: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "excludedCategories")]
            pub excluded_categories: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "syntheticDelays")]
            pub synthetic_delays: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "memoryDumpConfig")]
            pub memory_dump_config: Option<MemoryDumpConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct End(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCategories(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RecordClockSyncMarker {
            #[serde(default)]
            #[serde(rename = "syncId")]
            pub sync_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestMemoryDump {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "deterministic")]
            pub deterministic: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "levelOfDetail")]
            pub level_of_detail: Option<MemoryDumpLevelOfDetail>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Start {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "categories")]
            pub categories: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "options")]
            pub options: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "bufferUsageReportingInterval")]
            pub buffer_usage_reporting_interval: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "transferMode")]
            pub transfer_mode: Option<StartTransfer_modeOption>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "streamFormat")]
            pub stream_format: Option<StreamFormat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "streamCompression")]
            pub stream_compression: Option<StreamCompression>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "traceConfig")]
            pub trace_config: Option<TraceConfig>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "perfettoConfig")]
            pub perfetto_config: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "tracingBackend")]
            pub tracing_backend: Option<TracingBackend>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EndReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetCategoriesReturnObject {
            #[serde(rename = "categories")]
            pub categories: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RecordClockSyncMarkerReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestMemoryDumpReturnObject {
            #[serde(default)]
            #[serde(rename = "dumpGuid")]
            pub dump_guid: String,
            #[serde(default)]
            #[serde(rename = "success")]
            pub success: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartReturnObject {}
        impl Method for End {
            const NAME: &'static str = "Tracing.end";
            type ReturnObject = EndReturnObject;
        }
        impl Method for GetCategories {
            const NAME: &'static str = "Tracing.getCategories";
            type ReturnObject = GetCategoriesReturnObject;
        }
        impl Method for RecordClockSyncMarker {
            const NAME: &'static str = "Tracing.recordClockSyncMarker";
            type ReturnObject = RecordClockSyncMarkerReturnObject;
        }
        impl Method for RequestMemoryDump {
            const NAME: &'static str = "Tracing.requestMemoryDump";
            type ReturnObject = RequestMemoryDumpReturnObject;
        }
        impl Method for Start {
            const NAME: &'static str = "Tracing.start";
            type ReturnObject = StartReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct BufferUsageEvent {
                pub params: BufferUsageEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct BufferUsageEventParams {
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "percentFull")]
                pub percent_full: Option<JsFloat>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "eventCount")]
                pub event_count: Option<JsFloat>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "value")]
                pub value: Option<JsFloat>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DataCollectedEvent {
                pub params: DataCollectedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DataCollectedEventParams {}
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TracingCompleteEvent {
                pub params: TracingCompleteEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TracingCompleteEventParams {
                #[serde(default)]
                #[serde(rename = "dataLossOccurred")]
                pub data_loss_occurred: bool,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "stream")]
                pub stream: Option<super::super::IO::StreamHandle>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "traceFormat")]
                pub trace_format: Option<super::StreamFormat>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "streamCompression")]
                pub stream_compression: Option<super::StreamCompression>,
            }
        }
    }
    pub mod Fetch {
        use super::types::*;
        use super::Network;
        use super::Page;
        use super::IO;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type RequestId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum RequestStage {
            #[serde(rename = "Request")]
            Request,
            #[serde(rename = "Response")]
            Response,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum AuthChallengeSource {
            #[serde(rename = "Server")]
            Server,
            #[serde(rename = "Proxy")]
            Proxy,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum AuthChallengeResponseResponse {
            #[serde(rename = "Default")]
            Default,
            #[serde(rename = "CancelAuth")]
            CancelAuth,
            #[serde(rename = "ProvideCredentials")]
            ProvideCredentials,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RequestPattern {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "urlPattern")]
            pub url_pattern: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "resourceType")]
            pub resource_Type: Option<Network::ResourceType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "requestStage")]
            pub request_stage: Option<RequestStage>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct HeaderEntry {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AuthChallenge {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "source")]
            pub source: Option<AuthChallengeSource>,
            #[serde(default)]
            #[serde(rename = "origin")]
            pub origin: String,
            #[serde(default)]
            #[serde(rename = "scheme")]
            pub scheme: String,
            #[serde(default)]
            #[serde(rename = "realm")]
            pub realm: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AuthChallengeResponse {
            #[serde(rename = "response")]
            pub response: AuthChallengeResponseResponse,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "username")]
            pub username: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "password")]
            pub password: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Enable {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "patterns")]
            pub patterns: Option<Vec<RequestPattern>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "handleAuthRequests")]
            pub handle_auth_requests: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct FailRequest {
            #[serde(rename = "requestId")]
            pub request_id: RequestId,
            #[serde(rename = "errorReason")]
            pub error_reason: Network::ErrorReason,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct FulfillRequest {
            #[serde(rename = "requestId")]
            pub request_id: RequestId,
            #[serde(default)]
            #[serde(rename = "responseCode")]
            pub response_code: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "responseHeaders")]
            pub response_headers: Option<Vec<HeaderEntry>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "binaryResponseHeaders")]
            pub binary_response_headers: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "body")]
            pub body: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "responsePhrase")]
            pub response_phrase: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ContinueRequest {
            #[serde(rename = "requestId")]
            pub request_id: RequestId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "url")]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "method")]
            pub method: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "postData")]
            pub post_data: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "headers")]
            pub headers: Option<Vec<HeaderEntry>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "interceptResponse")]
            pub intercept_response: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ContinueWithAuth {
            #[serde(rename = "requestId")]
            pub request_id: RequestId,
            #[serde(rename = "authChallengeResponse")]
            pub auth_challenge_response: AuthChallengeResponse,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ContinueResponse {
            #[serde(rename = "requestId")]
            pub request_id: RequestId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "responseCode")]
            pub response_code: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "responsePhrase")]
            pub response_phrase: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "responseHeaders")]
            pub response_headers: Option<Vec<HeaderEntry>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "binaryResponseHeaders")]
            pub binary_response_headers: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetResponseBody {
            #[serde(rename = "requestId")]
            pub request_id: RequestId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TakeResponseBodyAsStream {
            #[serde(rename = "requestId")]
            pub request_id: RequestId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FailRequestReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FulfillRequestReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContinueRequestReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContinueWithAuthReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContinueResponseReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetResponseBodyReturnObject {
            #[serde(default)]
            #[serde(rename = "body")]
            pub body: String,
            #[serde(default)]
            #[serde(rename = "base64Encoded")]
            pub base_64_encoded: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct TakeResponseBodyAsStreamReturnObject {
            #[serde(rename = "stream")]
            pub stream: IO::StreamHandle,
        }
        impl Method for Disable {
            const NAME: &'static str = "Fetch.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Fetch.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for FailRequest {
            const NAME: &'static str = "Fetch.failRequest";
            type ReturnObject = FailRequestReturnObject;
        }
        impl Method for FulfillRequest {
            const NAME: &'static str = "Fetch.fulfillRequest";
            type ReturnObject = FulfillRequestReturnObject;
        }
        impl Method for ContinueRequest {
            const NAME: &'static str = "Fetch.continueRequest";
            type ReturnObject = ContinueRequestReturnObject;
        }
        impl Method for ContinueWithAuth {
            const NAME: &'static str = "Fetch.continueWithAuth";
            type ReturnObject = ContinueWithAuthReturnObject;
        }
        impl Method for ContinueResponse {
            const NAME: &'static str = "Fetch.continueResponse";
            type ReturnObject = ContinueResponseReturnObject;
        }
        impl Method for GetResponseBody {
            const NAME: &'static str = "Fetch.getResponseBody";
            type ReturnObject = GetResponseBodyReturnObject;
        }
        impl Method for TakeResponseBodyAsStream {
            const NAME: &'static str = "Fetch.takeResponseBodyAsStream";
            type ReturnObject = TakeResponseBodyAsStreamReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RequestPausedEvent {
                pub params: RequestPausedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RequestPausedEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "request")]
                pub request: super::super::Network::Request,
                #[serde(rename = "frameId")]
                pub frame_id: super::super::Page::FrameId,
                #[serde(rename = "resourceType")]
                pub resource_Type: super::super::Network::ResourceType,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "responseErrorReason")]
                pub response_error_reason: Option<super::super::Network::ErrorReason>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "responseStatusCode")]
                pub response_status_code: Option<JsUInt>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "responseStatusText")]
                pub response_status_text: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "responseHeaders")]
                pub response_headers: Option<Vec<super::HeaderEntry>>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(rename = "networkId")]
                pub network_id: Option<super::RequestId>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AuthRequiredEvent {
                pub params: AuthRequiredEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AuthRequiredEventParams {
                #[serde(rename = "requestId")]
                pub request_id: super::RequestId,
                #[serde(rename = "request")]
                pub request: super::super::Network::Request,
                #[serde(rename = "frameId")]
                pub frame_id: super::super::Page::FrameId,
                #[serde(rename = "resourceType")]
                pub resource_Type: super::super::Network::ResourceType,
                #[serde(rename = "authChallenge")]
                pub auth_challenge: super::AuthChallenge,
            }
        }
    }
    pub mod WebAudio {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type GraphObjectId = String;
        pub type NodeType = String;
        pub type ParamType = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ContextType {
            #[serde(rename = "realtime")]
            Realtime,
            #[serde(rename = "offline")]
            Offline,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ContextState {
            #[serde(rename = "suspended")]
            Suspended,
            #[serde(rename = "running")]
            Running,
            #[serde(rename = "closed")]
            Closed,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ChannelCountMode {
            #[serde(rename = "clamped-max")]
            ClampedMax,
            #[serde(rename = "explicit")]
            Explicit,
            #[serde(rename = "max")]
            Max,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum ChannelInterpretation {
            #[serde(rename = "discrete")]
            Discrete,
            #[serde(rename = "speakers")]
            Speakers,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum AutomationRate {
            #[serde(rename = "a-rate")]
            ARate,
            #[serde(rename = "k-rate")]
            KRate,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ContextRealtimeData {
            #[serde(default)]
            #[serde(rename = "currentTime")]
            pub current_time: JsFloat,
            #[serde(default)]
            #[serde(rename = "renderCapacity")]
            pub render_capacity: JsFloat,
            #[serde(default)]
            #[serde(rename = "callbackIntervalMean")]
            pub callback_interval_mean: JsFloat,
            #[serde(default)]
            #[serde(rename = "callbackIntervalVariance")]
            pub callback_interval_variance: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct BaseAudioContext {
            #[serde(rename = "contextId")]
            pub context_id: GraphObjectId,
            #[serde(rename = "contextType")]
            pub context_Type: ContextType,
            #[serde(rename = "contextState")]
            pub context_state: ContextState,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "realtimeData")]
            pub realtime_data: Option<ContextRealtimeData>,
            #[serde(default)]
            #[serde(rename = "callbackBufferSize")]
            pub callback_buffer_size: JsFloat,
            #[serde(default)]
            #[serde(rename = "maxOutputChannelCount")]
            pub max_output_channel_count: JsFloat,
            #[serde(default)]
            #[serde(rename = "sampleRate")]
            pub sample_rate: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AudioListener {
            #[serde(rename = "listenerId")]
            pub listener_id: GraphObjectId,
            #[serde(rename = "contextId")]
            pub context_id: GraphObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AudioNode {
            #[serde(rename = "nodeId")]
            pub node_id: GraphObjectId,
            #[serde(rename = "contextId")]
            pub context_id: GraphObjectId,
            #[serde(rename = "nodeType")]
            pub node_Type: NodeType,
            #[serde(default)]
            #[serde(rename = "numberOfInputs")]
            pub number_of_inputs: JsFloat,
            #[serde(default)]
            #[serde(rename = "numberOfOutputs")]
            pub number_of_outputs: JsFloat,
            #[serde(default)]
            #[serde(rename = "channelCount")]
            pub channel_count: JsFloat,
            #[serde(rename = "channelCountMode")]
            pub channel_count_mode: ChannelCountMode,
            #[serde(rename = "channelInterpretation")]
            pub channel_interpretation: ChannelInterpretation,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AudioParam {
            #[serde(rename = "paramId")]
            pub param_id: GraphObjectId,
            #[serde(rename = "nodeId")]
            pub node_id: GraphObjectId,
            #[serde(rename = "contextId")]
            pub context_id: GraphObjectId,
            #[serde(rename = "paramType")]
            pub param_Type: ParamType,
            #[serde(rename = "rate")]
            pub rate: AutomationRate,
            #[serde(default)]
            #[serde(rename = "defaultValue")]
            pub default_value: JsFloat,
            #[serde(default)]
            #[serde(rename = "minValue")]
            pub min_value: JsFloat,
            #[serde(default)]
            #[serde(rename = "maxValue")]
            pub max_value: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetRealtimeData {
            #[serde(rename = "contextId")]
            pub context_id: GraphObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetRealtimeDataReturnObject {
            #[serde(rename = "realtimeData")]
            pub realtime_data: ContextRealtimeData,
        }
        impl Method for Enable {
            const NAME: &'static str = "WebAudio.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "WebAudio.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for GetRealtimeData {
            const NAME: &'static str = "WebAudio.getRealtimeData";
            type ReturnObject = GetRealtimeDataReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ContextCreatedEvent {
                pub params: ContextCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ContextCreatedEventParams {
                #[serde(rename = "context")]
                pub context: super::BaseAudioContext,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ContextWillBeDestroyedEvent {
                pub params: ContextWillBeDestroyedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ContextWillBeDestroyedEventParams {
                #[serde(rename = "contextId")]
                pub context_id: super::GraphObjectId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ContextChangedEvent {
                pub params: ContextChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ContextChangedEventParams {
                #[serde(rename = "context")]
                pub context: super::BaseAudioContext,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioListenerCreatedEvent {
                pub params: AudioListenerCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioListenerCreatedEventParams {
                #[serde(rename = "listener")]
                pub listener: super::AudioListener,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioListenerWillBeDestroyedEvent {
                pub params: AudioListenerWillBeDestroyedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioListenerWillBeDestroyedEventParams {
                #[serde(rename = "contextId")]
                pub context_id: super::GraphObjectId,
                #[serde(rename = "listenerId")]
                pub listener_id: super::GraphObjectId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioNodeCreatedEvent {
                pub params: AudioNodeCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioNodeCreatedEventParams {
                #[serde(rename = "node")]
                pub node: super::AudioNode,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioNodeWillBeDestroyedEvent {
                pub params: AudioNodeWillBeDestroyedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioNodeWillBeDestroyedEventParams {
                #[serde(rename = "contextId")]
                pub context_id: super::GraphObjectId,
                #[serde(rename = "nodeId")]
                pub node_id: super::GraphObjectId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioParamCreatedEvent {
                pub params: AudioParamCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioParamCreatedEventParams {
                #[serde(rename = "param")]
                pub param: super::AudioParam,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioParamWillBeDestroyedEvent {
                pub params: AudioParamWillBeDestroyedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioParamWillBeDestroyedEventParams {
                #[serde(rename = "contextId")]
                pub context_id: super::GraphObjectId,
                #[serde(rename = "nodeId")]
                pub node_id: super::GraphObjectId,
                #[serde(rename = "paramId")]
                pub param_id: super::GraphObjectId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodesConnectedEvent {
                pub params: NodesConnectedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodesConnectedEventParams {
                #[serde(rename = "contextId")]
                pub context_id: super::GraphObjectId,
                #[serde(rename = "sourceId")]
                pub source_id: super::GraphObjectId,
                #[serde(rename = "destinationId")]
                pub destination_id: super::GraphObjectId,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "sourceOutputIndex")]
                pub source_output_index: Option<JsFloat>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "destinationInputIndex")]
                pub destination_input_index: Option<JsFloat>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodesDisconnectedEvent {
                pub params: NodesDisconnectedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodesDisconnectedEventParams {
                #[serde(rename = "contextId")]
                pub context_id: super::GraphObjectId,
                #[serde(rename = "sourceId")]
                pub source_id: super::GraphObjectId,
                #[serde(rename = "destinationId")]
                pub destination_id: super::GraphObjectId,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "sourceOutputIndex")]
                pub source_output_index: Option<JsFloat>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "destinationInputIndex")]
                pub destination_input_index: Option<JsFloat>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodeParamConnectedEvent {
                pub params: NodeParamConnectedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodeParamConnectedEventParams {
                #[serde(rename = "contextId")]
                pub context_id: super::GraphObjectId,
                #[serde(rename = "sourceId")]
                pub source_id: super::GraphObjectId,
                #[serde(rename = "destinationId")]
                pub destination_id: super::GraphObjectId,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "sourceOutputIndex")]
                pub source_output_index: Option<JsFloat>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodeParamDisconnectedEvent {
                pub params: NodeParamDisconnectedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodeParamDisconnectedEventParams {
                #[serde(rename = "contextId")]
                pub context_id: super::GraphObjectId,
                #[serde(rename = "sourceId")]
                pub source_id: super::GraphObjectId,
                #[serde(rename = "destinationId")]
                pub destination_id: super::GraphObjectId,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                #[serde(rename = "sourceOutputIndex")]
                pub source_output_index: Option<JsFloat>,
            }
        }
    }
    pub mod WebAuthn {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type AuthenticatorId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum AuthenticatorProtocol {
            #[serde(rename = "u2f")]
            U2F,
            #[serde(rename = "ctap2")]
            Ctap2,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum Ctap2Version {
            #[serde(rename = "ctap2_0")]
            Ctap20,
            #[serde(rename = "ctap2_1")]
            Ctap21,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum AuthenticatorTransport {
            #[serde(rename = "usb")]
            Usb,
            #[serde(rename = "nfc")]
            Nfc,
            #[serde(rename = "ble")]
            Ble,
            #[serde(rename = "cable")]
            Cable,
            #[serde(rename = "internal")]
            Internal,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct VirtualAuthenticatorOptions {
            #[serde(rename = "protocol")]
            pub protocol: AuthenticatorProtocol,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "ctap2Version")]
            pub ctap_2_version: Option<Ctap2Version>,
            #[serde(rename = "transport")]
            pub transport: AuthenticatorTransport,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "hasResidentKey")]
            pub has_resident_key: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "hasUserVerification")]
            pub has_user_verification: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "hasLargeBlob")]
            pub has_large_blob: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "hasCredBlob")]
            pub has_cred_blob: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "automaticPresenceSimulation")]
            pub automatic_presence_simulation: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "isUserVerified")]
            pub is_user_verified: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct Credential {
            #[serde(default)]
            #[serde(rename = "credentialId")]
            pub credential_id: String,
            #[serde(default)]
            #[serde(rename = "isResidentCredential")]
            pub is_resident_credential: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "rpId")]
            pub rp_id: Option<String>,
            #[serde(default)]
            #[serde(rename = "privateKey")]
            pub private_key: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "userHandle")]
            pub user_handle: Option<String>,
            #[serde(default)]
            #[serde(rename = "signCount")]
            pub sign_count: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(rename = "largeBlob")]
            pub large_blob: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AddVirtualAuthenticator {
            #[serde(rename = "options")]
            pub options: VirtualAuthenticatorOptions,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RemoveVirtualAuthenticator {
            #[serde(rename = "authenticatorId")]
            pub authenticator_id: AuthenticatorId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AddCredential {
            #[serde(rename = "authenticatorId")]
            pub authenticator_id: AuthenticatorId,
            #[serde(rename = "credential")]
            pub credential: Credential,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetCredential {
            #[serde(rename = "authenticatorId")]
            pub authenticator_id: AuthenticatorId,
            #[serde(default)]
            #[serde(rename = "credentialId")]
            pub credential_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetCredentials {
            #[serde(rename = "authenticatorId")]
            pub authenticator_id: AuthenticatorId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct RemoveCredential {
            #[serde(rename = "authenticatorId")]
            pub authenticator_id: AuthenticatorId,
            #[serde(default)]
            #[serde(rename = "credentialId")]
            pub credential_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct ClearCredentials {
            #[serde(rename = "authenticatorId")]
            pub authenticator_id: AuthenticatorId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetUserVerified {
            #[serde(rename = "authenticatorId")]
            pub authenticator_id: AuthenticatorId,
            #[serde(default)]
            #[serde(rename = "isUserVerified")]
            pub is_user_verified: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct SetAutomaticPresenceSimulation {
            #[serde(rename = "authenticatorId")]
            pub authenticator_id: AuthenticatorId,
            #[serde(default)]
            #[serde(rename = "enabled")]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct AddVirtualAuthenticatorReturnObject {
            #[serde(rename = "authenticatorId")]
            pub authenticator_id: AuthenticatorId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveVirtualAuthenticatorReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddCredentialReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetCredentialReturnObject {
            #[serde(rename = "credential")]
            pub credential: Credential,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct GetCredentialsReturnObject {
            #[serde(rename = "credentials")]
            pub credentials: Vec<Credential>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveCredentialReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearCredentialsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetUserVerifiedReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAutomaticPresenceSimulationReturnObject {}
        impl Method for Enable {
            const NAME: &'static str = "WebAuthn.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "WebAuthn.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for AddVirtualAuthenticator {
            const NAME: &'static str = "WebAuthn.addVirtualAuthenticator";
            type ReturnObject = AddVirtualAuthenticatorReturnObject;
        }
        impl Method for RemoveVirtualAuthenticator {
            const NAME: &'static str = "WebAuthn.removeVirtualAuthenticator";
            type ReturnObject = RemoveVirtualAuthenticatorReturnObject;
        }
        impl Method for AddCredential {
            const NAME: &'static str = "WebAuthn.addCredential";
            type ReturnObject = AddCredentialReturnObject;
        }
        impl Method for GetCredential {
            const NAME: &'static str = "WebAuthn.getCredential";
            type ReturnObject = GetCredentialReturnObject;
        }
        impl Method for GetCredentials {
            const NAME: &'static str = "WebAuthn.getCredentials";
            type ReturnObject = GetCredentialsReturnObject;
        }
        impl Method for RemoveCredential {
            const NAME: &'static str = "WebAuthn.removeCredential";
            type ReturnObject = RemoveCredentialReturnObject;
        }
        impl Method for ClearCredentials {
            const NAME: &'static str = "WebAuthn.clearCredentials";
            type ReturnObject = ClearCredentialsReturnObject;
        }
        impl Method for SetUserVerified {
            const NAME: &'static str = "WebAuthn.setUserVerified";
            type ReturnObject = SetUserVerifiedReturnObject;
        }
        impl Method for SetAutomaticPresenceSimulation {
            const NAME: &'static str = "WebAuthn.setAutomaticPresenceSimulation";
            type ReturnObject = SetAutomaticPresenceSimulationReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod Media {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type PlayerId = String;
        pub type Timestamp = JsFloat;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum PlayerMessageLevel {
            #[serde(rename = "error")]
            Error,
            #[serde(rename = "warning")]
            Warning,
            #[serde(rename = "info")]
            Info,
            #[serde(rename = "debug")]
            Debug,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub enum PlayerErrorType {
            #[serde(rename = "pipeline_error")]
            PipelineError,
            #[serde(rename = "media_error")]
            MediaError,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PlayerMessage {
            #[serde(rename = "level")]
            pub level: PlayerMessageLevel,
            #[serde(default)]
            #[serde(rename = "message")]
            pub message: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PlayerProperty {
            #[serde(default)]
            #[serde(rename = "name")]
            pub name: String,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PlayerEvent {
            #[serde(rename = "timestamp")]
            pub timestamp: Timestamp,
            #[serde(default)]
            #[serde(rename = "value")]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        pub struct PlayerError {
            #[serde(rename = "type")]
            pub Type: PlayerErrorType,
            #[serde(default)]
            #[serde(rename = "errorCode")]
            pub error_code: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        impl Method for Enable {
            const NAME: &'static str = "Media.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Media.disable";
            type ReturnObject = DisableReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PlayerPropertiesChangedEvent {
                pub params: PlayerPropertiesChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PlayerPropertiesChangedEventParams {
                #[serde(rename = "playerId")]
                pub player_id: super::PlayerId,
                #[serde(rename = "properties")]
                pub properties: Vec<super::PlayerProperty>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PlayerEventsAddedEvent {
                pub params: PlayerEventsAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PlayerEventsAddedEventParams {
                #[serde(rename = "playerId")]
                pub player_id: super::PlayerId,
                #[serde(rename = "events")]
                pub events: Vec<super::PlayerEvent>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PlayerMessagesLoggedEvent {
                pub params: PlayerMessagesLoggedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PlayerMessagesLoggedEventParams {
                #[serde(rename = "playerId")]
                pub player_id: super::PlayerId,
                #[serde(rename = "messages")]
                pub messages: Vec<super::PlayerMessage>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PlayerErrorsRaisedEvent {
                pub params: PlayerErrorsRaisedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PlayerErrorsRaisedEventParams {
                #[serde(rename = "playerId")]
                pub player_id: super::PlayerId,
                #[serde(rename = "errors")]
                pub errors: Vec<super::PlayerError>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PlayersCreatedEvent {
                pub params: PlayersCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PlayersCreatedEventParams {
                #[serde(rename = "players")]
                pub players: Vec<super::PlayerId>,
            }
        }
    }
}
