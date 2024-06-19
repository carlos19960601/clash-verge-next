"use client";

import { PropsWithChildren } from "react";
import { ErrorBoundary, FallbackProps } from "react-error-boundary";

const ErrorFallback = ({ error }: FallbackProps) => {
  return (
    <div>
      <h4>Something went wrong:(</h4>

      <pre>{error.message}</pre>

      <details title="Error Stack">
        <summary>Error Stack</summary>
        <pre>{error.stack}</pre>
      </details>
    </div>
  );
};

export const BaseErrorBoundary = ({ children }: PropsWithChildren) => {
  return (
    <ErrorBoundary FallbackComponent={ErrorFallback}>{children}</ErrorBoundary>
  );
};
