import { useLocation, Link } from 'react-router';
import { Result, Button } from 'antd';

export default function NotFound() {
  const location = useLocation();
  
  // Handle system/browser requests silently (return empty response)
  if (location.pathname.startsWith('/.well-known/') || 
      location.pathname.includes('favicon.ico') ||
      location.pathname.includes('chrome-extension://') ||
      location.pathname.includes('.json') ||
      location.pathname.includes('devtools') ||
      location.pathname.includes('appspecific')) {
    // Return empty div for system requests to avoid console errors
    return <div />;
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <Result
        status="404"
        title="404"
        subTitle={`页面 "${location.pathname}" 不存在`}
        extra={
          <Link to="/">
            <Button type="primary">返回首页</Button>
          </Link>
        }
      />
    </div>
  );
}