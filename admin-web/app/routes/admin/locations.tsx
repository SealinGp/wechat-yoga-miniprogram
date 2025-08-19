import { useState, useEffect } from 'react';
import { 
  Table, 
  Card, 
  Button, 
  Space, 
  Modal, 
  Form, 
  Input, 
  InputNumber, 
  Switch,
  message,
  Popconfirm,
  Tag,
  Typography,
  Select
} from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, EnvironmentOutlined, InfoCircleOutlined } from '@ant-design/icons';
import { api } from '../../utils/auth';

const { TextArea } = Input;
const { Text } = Typography;

interface Location {
  id: number;
  name: string;
  description?: string;
  capacity: number;
  equipment?: string[];
  facilities?: string[];
  floor_number: number;
  room_number: string;
  is_accessible?: boolean;
  booking_enabled?: boolean;
  hourly_rate?: number;
  images?: string[];
  is_active?: boolean;
  created_at?: string;
}

export default function Locations() {
  const [locations, setLocations] = useState<Location[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingLocation, setEditingLocation] = useState<Location | null>(null);
  const [form] = Form.useForm();

  const fetchLocations = async () => {
    try {
      setLoading(true);
      const response = await api.get('/locations');
      setLocations(response.data);
    } catch (error) {
      message.error('获取场地列表失败');
      console.error('Error fetching locations:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchLocations();
  }, []);

  const handleAdd = () => {
    setEditingLocation(null);
    form.resetFields();
    setModalVisible(true);
  };

  const handleEdit = (location: Location) => {
    setEditingLocation(location);
    form.setFieldsValue({
      name: location.name,
      description: location.description,
      capacity: location.capacity,
      equipment: location.equipment,
      facilities: location.facilities,
      floor_number: location.floor_number,
      room_number: location.room_number,
      is_accessible: location.is_accessible,
      booking_enabled: location.booking_enabled,
      hourly_rate: location.hourly_rate,
      is_active: location.is_active
    });
    setModalVisible(true);
  };

  const handleDelete = async (id: number, name: string) => {
    try {
      await api.delete(`/admin/locations/${id}`);
      message.success(`场地"${name}"删除成功`);
      fetchLocations();
    } catch (error: any) {
      message.error('删除场地失败');
      console.error('Error deleting location:', error);
    }
  };

  const handleSubmit = async (values: any) => {
    try {
      if (editingLocation) {
        // Update location
        await api.put(`/admin/locations/${editingLocation.id}`, values);
        message.success('场地更新成功');
      } else {
        // Create new location
        await api.post('/admin/locations', values);
        message.success('场地创建成功');
      }
      setModalVisible(false);
      fetchLocations();
    } catch (error: any) {
      message.error(editingLocation ? '更新场地失败' : '创建场地失败');
      console.error('Error saving location:', error);
    }
  };

  const columns = [
    {
      title: '场地名称',
      dataIndex: 'name',
      key: 'name',
      render: (name: string) => (
        <Space>
          <EnvironmentOutlined />
          <Text strong>{name}</Text>
        </Space>
      ),
    },
    {
      title: '楼层房间',
      key: 'location',
      render: (_, record: Location) => (
        <Text>{record.floor_number}F - {record.room_number}</Text>
      ),
    },
    {
      title: '容量',
      dataIndex: 'capacity',
      key: 'capacity',
      align: 'center',
    },
    {
      title: '时价',
      dataIndex: 'hourly_rate',
      key: 'hourly_rate',
      render: (rate: number) => rate ? `¥${rate}` : '-',
      align: 'center',
    },
    {
      title: '无障碍',
      dataIndex: 'is_accessible',
      key: 'is_accessible',
      render: (accessible: boolean) => (
        <Tag color={accessible ? 'green' : 'orange'}>
          {accessible ? '是' : '否'}
        </Tag>
      ),
      align: 'center',
    },
    {
      title: '预约状态',
      dataIndex: 'booking_enabled',
      key: 'booking_enabled',
      render: (enabled: boolean) => (
        <Tag color={enabled ? 'blue' : 'default'}>
          {enabled ? '开启' : '关闭'}
        </Tag>
      ),
      align: 'center',
    },
    {
      title: '状态',
      dataIndex: 'is_active',
      key: 'is_active',
      render: (is_active: boolean) => (
        <Tag color={is_active ? 'green' : 'red'}>
          {is_active ? '激活' : '停用'}
        </Tag>
      ),
      align: 'center',
    },
    {
      title: '操作',
      key: 'actions',
      render: (_, record: Location) => (
        <Space>
          <Button
            type="link"
            icon={<EditOutlined />}
            onClick={() => handleEdit(record)}
          >
            编辑
          </Button>
          <Popconfirm
            title={`确定要删除场地"${record.name}"吗？`}
            onConfirm={() => handleDelete(record.id, record.name)}
          >
            <Button
              type="link"
              danger
              icon={<DeleteOutlined />}
            >
              删除
            </Button>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <>
      <Card
        title="场地管理"
        extra={
          <Button
            type="primary"
            icon={<PlusOutlined />}
            onClick={handleAdd}
          >
            添加场地
          </Button>
        }
      >
        <Table
          columns={columns}
          dataSource={locations}
          rowKey="id"
          loading={loading}
          pagination={{
            showSizeChanger: true,
            showTotal: (total) => `共 ${total} 个场地`,
          }}
        />

        <Modal
          title={editingLocation ? '编辑场地' : '添加场地'}
          open={modalVisible}
          onCancel={() => setModalVisible(false)}
          footer={null}
          destroyOnClose
          width={700}
        >
          <Form
            form={form}
            layout="vertical"
            onFinish={handleSubmit}
          >
            <Form.Item
              name="name"
              label="场地名称"
              rules={[
                { required: true, message: '请输入场地名称' },
                { max: 255, message: '名称不能超过255个字符' }
              ]}
            >
              <Input placeholder="请输入场地名称" />
            </Form.Item>

            <Form.Item
              name="description"
              label="场地描述"
            >
              <TextArea rows={3} placeholder="请输入场地描述" />
            </Form.Item>

            <Space.Compact style={{ display: 'flex', width: '100%' }}>
              <Form.Item
                name="floor_number"
                label="楼层"
                style={{ flex: 1, marginRight: 8 }}
                rules={[{ required: true, message: '必填' }]}
              >
                <InputNumber placeholder="楼层" min={-5} max={50} style={{ width: '100%' }} />
              </Form.Item>

              <Form.Item
                name="room_number"
                label="房间号"
                style={{ flex: 1, marginLeft: 8 }}
                rules={[{ required: true, message: '必填' }]}
              >
                <Input placeholder="房间号" />
              </Form.Item>
            </Space.Compact>

            <Space.Compact style={{ display: 'flex', width: '100%' }}>
              <Form.Item
                name="capacity"
                label="容量"
                style={{ flex: 1, marginRight: 8 }}
                rules={[{ required: true, message: '必填' }]}
              >
                <InputNumber placeholder="最大容量" min={1} style={{ width: '100%' }} />
              </Form.Item>

              <Form.Item
                name="hourly_rate"
                label="时价 (¥)"
                style={{ flex: 1, marginLeft: 8 }}
              >
                <InputNumber placeholder="每小时价格" min={0} step={0.01} style={{ width: '100%' }} />
              </Form.Item>
            </Space.Compact>

            <Form.Item
              name="equipment"
              label="设备"
            >
              <Select
                mode="tags"
                placeholder="添加设备（输入后回车）"
                style={{ width: '100%' }}
              />
            </Form.Item>

            <Form.Item
              name="facilities"
              label="设施"
            >
              <Select
                mode="tags"
                placeholder="添加设施（输入后回车）"
                style={{ width: '100%' }}
              />
            </Form.Item>

            <Space size="large">
              <Form.Item
                name="is_accessible"
                label="无障碍通道"
                valuePropName="checked"
              >
                <Switch checkedChildren="是" unCheckedChildren="否" />
              </Form.Item>

              <Form.Item
                name="booking_enabled"
                label="开启预约"
                valuePropName="checked"
              >
                <Switch checkedChildren="开启" unCheckedChildren="关闭" />
              </Form.Item>

              {editingLocation && (
                <Form.Item
                  name="is_active"
                  label="状态"
                  valuePropName="checked"
                >
                  <Switch checkedChildren="激活" unCheckedChildren="停用" />
                </Form.Item>
              )}
            </Space>

            <Form.Item>
              <Space>
                <Button type="primary" htmlType="submit">
                  {editingLocation ? '更新' : '创建'}
                </Button>
                <Button onClick={() => setModalVisible(false)}>
                  取消
                </Button>
              </Space>
            </Form.Item>
          </Form>
        </Modal>
      </Card>
    </>
  );
}