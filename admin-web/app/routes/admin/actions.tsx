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
  Image
} from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { api, mockApi } from '../../utils/auth';

// Use real API now
const apiClient = api;

interface ActionButton {
  id: number;
  name: string;
  icon: string;
  link: string;
  sort_order: number;
  is_active: boolean;
  created_at: string;
}

export default function Actions() {
  const [actions, setActions] = useState<ActionButton[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingAction, setEditingAction] = useState<ActionButton | null>(null);
  const [form] = Form.useForm();

  const fetchActions = async () => {
    try {
      setLoading(true);
      const response = await apiClient.get('/admin/actions');
      setActions(response.data);
    } catch (error) {
      message.error('获取功能按钮列表失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchActions();
  }, []);

  const handleCreate = () => {
    setEditingAction(null);
    form.resetFields();
    form.setFieldsValue({
      sort_order: actions.length + 1,
      is_active: true,
    });
    setModalVisible(true);
  };

  const handleEdit = (record: ActionButton) => {
    setEditingAction(record);
    form.setFieldsValue(record);
    setModalVisible(true);
  };

  const handleDelete = async (id: number) => {
    try {
      await apiClient.delete(`/admin/actions/${id}`);
      message.success('删除成功');
      fetchActions();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      
      if (editingAction) {
        await apiClient.put(`/admin/actions/${editingAction.id}`, values);
        message.success('更新成功');
      } else {
        await apiClient.post('/admin/actions', values);
        message.success('创建成功');
      }
      
      setModalVisible(false);
      fetchActions();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const columns = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 80,
    },
    {
      title: '名称',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '图标',
      dataIndex: 'icon',
      key: 'icon',
      render: (icon: string) => (
        <Image
          src={icon}
          alt="图标"
          width={32}
          height={32}
          fallback="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAMIAAADDCAYAAADQvc6UAAABRWlDQ1BJQ0MgUHJvZmlsZQAAKJFjYGASSSwoyGFhYGDIzSspCnJ3UoiIjFJgf8LAwSDCIMogwMCcmFxc4BgQ4ANUwgCjUcG3awyMIPqyLsis7PPOq3QdDFcvjV3jOD1boQVTPQrgSkktTgbSf4A4LbmgqISBgTEFyFYuLykAsTuAbJEioKOA7DkgdjqEvQHEToKwj4DVhAQ5A9k3gGyB5IxEoBmML4BsnSQk8XQkNtReEOBxcfXxUQg1Mjc0dyHgXNJBSWpFCYh2zi+oLMpMzyhRcASGUqqCZ16yno6CkYGRAQMDKMwhqj/fAIcloxgHQqxAjIHBEugw5sUIsSQpBobtQPdLciLEVJYzMPBHMDBsayhILEqEO4DxG0txmrERhM29nYGBddr//5/DGRjYNRkY/l7////39v///y4Dmn+LgeHANwDrkl1AuO+pmgAAADhlWElmTU0AKgAAAAgAAYdpAAQAAAABAAAAGgAAAAAAAqACAAQAAAABAAAAwqADAAQAAAABAAAAwwAAAAD9b/HnAAAHlklEQVR4Ae3dP3Ik1RnG4W+FgYxN"
        />
      ),
    },
    {
      title: '链接',
      dataIndex: 'link',
      key: 'link',
    },
    {
      title: '排序',
      dataIndex: 'sort_order',
      key: 'sort_order',
      width: 80,
    },
    {
      title: '状态',
      dataIndex: 'is_active',
      key: 'is_active',
      render: (active: boolean) => (
        <Switch checked={active} disabled />
      ),
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
      render: (date: string) => new Date(date).toLocaleString(),
    },
    {
      title: '操作',
      key: 'action',
      render: (_, record: ActionButton) => (
        <Space>
          <Button
            type="primary"
            size="small"
            icon={<EditOutlined />}
            onClick={() => handleEdit(record)}
          >
            编辑
          </Button>
          <Popconfirm
            title="确定要删除这个功能按钮吗？"
            onConfirm={() => handleDelete(record.id)}
            okText="确定"
            cancelText="取消"
          >
            <Button
              type="primary"
              danger
              size="small"
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
        title="功能按钮管理"
        extra={
          <Button
            type="primary"
            icon={<PlusOutlined />}
            onClick={handleCreate}
          >
            新建按钮
          </Button>
        }
      >
        <Table
          columns={columns}
          dataSource={actions}
          rowKey="id"
          loading={loading}
          pagination={{ pageSize: 10 }}
        />
      </Card>

      <Modal
        title={editingAction ? '编辑功能按钮' : '新建功能按钮'}
        open={modalVisible}
        onOk={handleSubmit}
        onCancel={() => setModalVisible(false)}
        width={600}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            label="按钮名称"
            name="name"
            rules={[{ required: true, message: '请输入按钮名称' }]}
          >
            <Input placeholder="例如：瑜伽" />
          </Form.Item>

          <Form.Item
            label="图标链接"
            name="icon"
            rules={[{ required: true, message: '请输入图标链接' }]}
          >
            <Input placeholder="例如：https://cdn.example.com/icons/yoga.png" />
          </Form.Item>

          <Form.Item
            label="跳转链接"
            name="link"
            rules={[{ required: true, message: '请输入跳转链接' }]}
          >
            <Input placeholder="例如：/pages/lessons/lessons" />
          </Form.Item>

          <Form.Item
            label="排序顺序"
            name="sort_order"
            rules={[{ required: true, message: '请输入排序顺序' }]}
          >
            <InputNumber min={1} max={100} placeholder="数字越小排序越前" className="w-full" />
          </Form.Item>

          <Form.Item
            label="是否启用"
            name="is_active"
            valuePropName="checked"
          >
            <Switch />
          </Form.Item>
        </Form>
      </Modal>
    </>
  );
}