import { Card, Row, Col, Statistic } from 'antd';
import {
  UserOutlined,
  TeamOutlined,
  FileImageOutlined,
  NotificationOutlined,
} from '@ant-design/icons';

export default function Dashboard() {
  return (
    <div className="space-y-6">
        <div>
          <h1 className="text-2xl font-bold text-gray-900 mb-2">仪表盘</h1>
          <p className="text-gray-600">LC PILATES 空中普拉提管理系统</p>
        </div>

        <Row gutter={[16, 16]}>
          <Col xs={24} sm={12} lg={6}>
            <Card>
              <Statistic
                title="总用户数"
                value={156}
                prefix={<UserOutlined />}
                valueStyle={{ color: '#1890ff' }}
              />
            </Card>
          </Col>
          
          <Col xs={24} sm={12} lg={6}>
            <Card>
              <Statistic
                title="教师数量"
                value={8}
                prefix={<TeamOutlined />}
                valueStyle={{ color: '#52c41a' }}
              />
            </Card>
          </Col>
          
          <Col xs={24} sm={12} lg={6}>
            <Card>
              <Statistic
                title="轮播图"
                value={5}
                prefix={<FileImageOutlined />}
                valueStyle={{ color: '#faad14' }}
              />
            </Card>
          </Col>
          
          <Col xs={24} sm={12} lg={6}>
            <Card>
              <Statistic
                title="公告数量"
                value={12}
                prefix={<NotificationOutlined />}
                valueStyle={{ color: '#f5222d' }}
              />
            </Card>
          </Col>
        </Row>

        <Row gutter={[16, 16]}>
          <Col xs={24} lg={16}>
            <Card title="系统概览" className="h-96">
              <div className="h-full flex items-center justify-center text-gray-500">
                <div className="text-center">
                  <h3 className="text-lg font-medium mb-2">欢迎使用管理系统</h3>
                  <p>这里是LC PILATES空中普拉提的后台管理系统</p>
                  <p className="mt-4 text-sm">您可以通过左侧菜单管理各个模块</p>
                </div>
              </div>
            </Card>
          </Col>
          
          <Col xs={24} lg={8}>
            <Card title="快捷操作" className="h-96">
              <div className="space-y-4">
                <Card.Meta
                  avatar={<UserOutlined className="text-blue-500" />}
                  title="功能按钮管理"
                  description="管理首页的功能按钮"
                />
                <Card.Meta
                  avatar={<TeamOutlined className="text-green-500" />}
                  title="教师管理"
                  description="添加和管理教师信息"
                />
                <Card.Meta
                  avatar={<FileImageOutlined className="text-yellow-500" />}
                  title="轮播图管理"
                  description="管理首页轮播图片"
                />
                <Card.Meta
                  avatar={<NotificationOutlined className="text-red-500" />}
                  title="公告管理"
                  description="发布和管理系统公告"
                />
              </div>
            </Card>
          </Col>
        </Row>
    </div>
  );
}