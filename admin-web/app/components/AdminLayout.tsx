import { useState, useEffect } from 'react';
import { useNavigate, useLocation, Outlet } from 'react-router';
import { Layout, Menu, Button, Dropdown, Avatar, Space, message } from 'antd';
import {
  DashboardOutlined,
  AppstoreOutlined,
  TeamOutlined,
  FileImageOutlined,
  NotificationOutlined,
  LogoutOutlined,
  UserOutlined,
  MenuFoldOutlined,
  MenuUnfoldOutlined,
  SettingOutlined,
  EnvironmentOutlined,
  CalendarOutlined,
} from '@ant-design/icons';
import { getStoredAuth, clearStoredAuth } from '../utils/auth';

const { Header, Sider, Content } = Layout;

export default function AdminLayout() {
  const [collapsed, setCollapsed] = useState(false);
  const [user, setUser] = useState<any>(null);
  const navigate = useNavigate();
  const location = useLocation();

  useEffect(() => {
    const { user: storedUser, token } = getStoredAuth();
    if (!token) {
      navigate('/login');
      return;
    }
    setUser(storedUser);
  }, [navigate]);

  const handleLogout = () => {
    clearStoredAuth();
    message.success('已退出登录');
    navigate('/login');
  };

  const menuItems = [
    {
      key: '/admin/dashboard',
      icon: <DashboardOutlined />,
      label: '仪表盘',
    },
    {
      key: '/admin/lessions',
      icon: <CalendarOutlined />,
      label: '课程管理',
    },
    {
      key: '/admin/actions',
      icon: <AppstoreOutlined />,
      label: '功能按钮',
    },
    {
      key: '/admin/teachers',
      icon: <TeamOutlined />,
      label: '教师管理',
    },
    {
      key: '/admin/posters',
      icon: <FileImageOutlined />,
      label: '轮播图',
    },
    {
      key: '/admin/notices',
      icon: <NotificationOutlined />,
      label: '公告管理',
    },
    {
      key: '/admin/locations',
      icon: <EnvironmentOutlined />,
      label: '场地管理',
    },
    {
      key: '/admin/users',
      icon: <UserOutlined />,
      label: '用户管理',
    },
    {
      key: '/admin/admin-users',
      icon: <SettingOutlined />,
      label: '管理员用户',
    },
  ];

  const userMenuItems = [
    {
      key: 'profile',
      icon: <UserOutlined />,
      label: '个人信息',
    },
    {
      type: 'divider' as const,
    },
    {
      key: 'logout',
      icon: <LogoutOutlined />,
      label: '退出登录',
      onClick: handleLogout,
    },
  ];

  const handleMenuClick = ({ key }: { key: string }) => {
    navigate(key);
  };

  if (!user) {
    return null; // or loading spinner
  }

  return (
    <Layout className="min-h-screen h-full">
      <Sider trigger={null} collapsible collapsed={collapsed} className="bg-white">
        <div className="h-16 flex items-center justify-center border-b">
          <h1 className="text-lg font-bold text-blue-600">
            {collapsed ? 'LC' : 'LC PILATES'}
          </h1>
        </div>
        <Menu
          theme="light"
          mode="inline"
          selectedKeys={[location.pathname]}
          items={menuItems}
          onClick={handleMenuClick}
          className="border-r-0"
        />
      </Sider>

      <Layout>
        <Header className="bg-white px-4 flex items-center justify-between shadow-sm">
          <Button
            type="text"
            icon={collapsed ? <MenuUnfoldOutlined /> : <MenuFoldOutlined />}
            onClick={() => setCollapsed(!collapsed)}
            className="text-base"
          />

          <Dropdown
            menu={{
              items: userMenuItems,
              onClick: ({ key }) => {
                if (key === 'logout') {
                  handleLogout();
                }
              },
            }}
            placement="bottomRight"
          >
            <Space className="cursor-pointer">
              <Avatar size="small" icon={<UserOutlined />} />
              <span>{user?.username}</span>
            </Space>
          </Dropdown>
        </Header>

        <Content className="p-6 bg-gray-50 overflow-auto">
          <Outlet />
        </Content>
      </Layout>
    </Layout>
  );
}