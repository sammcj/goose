import React, { useState, useEffect } from 'react';
import { Input } from '../../ui/input';
import { Check, Lock } from 'lucide-react';
import { Switch } from '../../ui/switch';

export default function SessionSharingSection() {
  const envBaseUrlShare = window.appConfig.get('GOOSE_BASE_URL_SHARE');
  console.log('envBaseUrlShare', envBaseUrlShare);

  // If env is set, force sharing enabled and set the baseUrl accordingly.
  const [sessionSharingConfig, setSessionSharingConfig] = useState({
    enabled: envBaseUrlShare ? true : false,
    baseUrl: typeof envBaseUrlShare === 'string' ? envBaseUrlShare : '',
  });
  const [urlError, setUrlError] = useState('');
  // isUrlConfigured is true if the user has configured a baseUrl and it is valid.
  const isUrlConfigured =
    !envBaseUrlShare &&
    sessionSharingConfig.enabled &&
    isValidUrl(String(sessionSharingConfig.baseUrl));

  // Only load saved config from localStorage if the env variable is not provided.
  useEffect(() => {
    if (envBaseUrlShare) {
      // If env variable is set, save the forced configuration to localStorage
      const forcedConfig = {
        enabled: true,
        baseUrl: typeof envBaseUrlShare === 'string' ? envBaseUrlShare : '',
      };
      localStorage.setItem('session_sharing_config', JSON.stringify(forcedConfig));
    } else {
      const savedSessionConfig = localStorage.getItem('session_sharing_config');
      if (savedSessionConfig) {
        try {
          const config = JSON.parse(savedSessionConfig);
          setSessionSharingConfig(config);
        } catch (error) {
          console.error('Error parsing session sharing config:', error);
        }
      }
    }
  }, [envBaseUrlShare]);

  // Helper to check if the user's input is a valid URL
  function isValidUrl(value: string): boolean {
    if (!value) return false;
    try {
      new URL(value);
      return true;
    } catch {
      return false;
    }
  }

  // Toggle sharing (only allowed when env is not set).
  const toggleSharing = () => {
    if (envBaseUrlShare) {
      return; // Do nothing if the environment variable forces sharing.
    }
    setSessionSharingConfig((prev) => {
      const updated = { ...prev, enabled: !prev.enabled };
      localStorage.setItem('session_sharing_config', JSON.stringify(updated));
      return updated;
    });
  };

  // Handle changes to the base URL field
  const handleBaseUrlChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newBaseUrl = e.target.value;
    setSessionSharingConfig((prev) => ({
      ...prev,
      baseUrl: newBaseUrl,
    }));

    if (isValidUrl(newBaseUrl)) {
      setUrlError('');
      const updated = { ...sessionSharingConfig, baseUrl: newBaseUrl };
      localStorage.setItem('session_sharing_config', JSON.stringify(updated));
    } else {
      setUrlError('Invalid URL format. Please enter a valid URL (e.g. https://example.com/api).');
    }
  };

  return (
    <section id="session-sharing" className="px-8">
      {/*Title*/}
      <div className="flex justify-between items-center mb-2">
        <h2 className="text-xl font-medium text-textStandard">Session sharing</h2>
      </div>

      <div className="border-b border-borderSubtle pb-8">
        {envBaseUrlShare ? (
          <p className="text-sm text-textStandard mb-4">
            Session sharing is configured but fully opt-in — your sessions are only shared when you
            explicitly click the share button.
          </p>
        ) : (
          <p className="text-sm text-textStandard mb-4">
            You can enable session sharing to share your sessions with others. You'll then need to
            enter the base URL for the session sharing API endpoint. Anyone with access to the same
            API and sharing session enabled will be able to see your sessions.
          </p>
        )}

        <div className="space-y-4">
          {/* Toggle for enabling session sharing */}
          <div className="flex items-center justify-between">
            <label className="text-textStandard cursor-pointer">
              {envBaseUrlShare
                ? 'Session sharing has already been configured'
                : 'Enable session sharing'}
            </label>
            {envBaseUrlShare ? (
              <Lock className="w-5 h-5 text-gray-600" />
            ) : (
              <Switch
                checked={sessionSharingConfig.enabled}
                disabled={!!envBaseUrlShare}
                onCheckedChange={toggleSharing}
                variant="mono"
              />
            )}
          </div>

          {/* Base URL field (only visible if enabled) */}
          {sessionSharingConfig.enabled && (
            <div className="space-y-2 relative">
              <div className="flex items-center space-x-2">
                <label
                  htmlFor="session-sharing-url"
                  className="text-sm font-medium text-textStandard"
                >
                  Base URL
                </label>
                {isUrlConfigured && <Check className="w-5 h-5 text-green-500" />}
              </div>
              <div className="flex items-center">
                <Input
                  id="session-sharing-url"
                  type="url"
                  placeholder="https://example.com/api"
                  value={sessionSharingConfig.baseUrl}
                  disabled={!!envBaseUrlShare}
                  {...(envBaseUrlShare ? {} : { onChange: handleBaseUrlChange })}
                />
              </div>
              {urlError && <p className="text-red-500 text-sm">{urlError}</p>}
            </div>
          )}
        </div>
      </div>
    </section>
  );
}
